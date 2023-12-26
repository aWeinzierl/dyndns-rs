#![feature(async_closure)]

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use error::Error;
use futures_retry::FutureRetry;
use preferences::{AppInfo, Preferences};
use public_ip::{addr_v6, addr_v4};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumDiscriminants, EnumString};

use dns_record_list::DnsRecordList;
use godaddy::RecordType;
use godaddy_handler::GoDaddyAuthenticationData;
use retry_handler::RetryHandler;

use crate::update_handler::{CreatableUpdateHandler, DnsRecord, UpdateHandler};

mod retry_handler;
mod dns_record_list;
mod error;
mod godaddy_handler;
mod update_handler;

fn retry_handler() -> RetryHandler {
    RetryHandler::new(3, 100)
}

type AuthenticationDataList = Vec<AuthenticationData>;

#[derive(Serialize, Deserialize, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, AsRefStr, Hash))]
enum AuthenticationData {
    GoDaddy(GoDaddyAuthenticationData),
}

#[derive(Serialize, Deserialize, Debug)]
struct IPs {
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}

const APP_INFO: AppInfo = AppInfo {
    name: "DynDns Service for Godaddy",
    author: "Andreas Weinzierl",
};
const AUTH_KEY: &str = "authentication";
const DNS_ENTRIES_KEY: &str = "dns-entries";
const IP_KEY: &str = "ips";

fn
create_handlers(data: AuthenticationData)
    -> (AuthenticationDataDiscriminants, Box<dyn UpdateHandler>) {
    let discriminant = AuthenticationDataDiscriminants::from(&data);

    let handler = match data {
        AuthenticationData::GoDaddy(v) =>
            godaddy_handler::GodaddyHandler::new(v)
    };

    (discriminant, handler)
}

async fn
get_ip_address_by_resolver<Resolver, AddrFuture, AddrType>(resolve: Resolver) -> Option<AddrType> 
where 
    Resolver: Fn() -> AddrFuture + Clone + Send + 'static,
    AddrFuture: Future<Output = Option<AddrType>> + Send + 'static,
    {
    let ip = FutureRetry::new(
        async || resolve().await.ok_or(()), 
        retry_handler(),
    ).await.ok()?.0;
    Some(ip)
}

fn collect_record_types(dns_record_list: &DnsRecordList) -> HashSet<RecordType> {
    let mut set: HashSet<RecordType> = HashSet::new();
    for service in dns_record_list{
        for specification in &service.specifications {
            for specification in &specification.specifications {
                for specification in &specification.specifications {
                    set.insert(specification.record_type);
                }
            }
        }
    }
    set
}

fn generate_should_be_processed(records: &HashSet<RecordType>) -> Box<dyn Fn(RecordType) -> bool> {
    match records.len() {
        1 => {
            let record_in_set = *records.iter().next().unwrap();
            Box::new(move |record: RecordType| record_in_set == record)
        }
        2 => Box::new(|_| true),
        _ => panic!("Got three different record types, although only 2 should exist")
    }
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let dns_entries: Vec<dns_record_list::ServiceSpecifications> =
        DnsRecordList::load(&APP_INFO, DNS_ENTRIES_KEY)?;

    let mut records = collect_record_types(&dns_entries);
    if records.is_empty() { return Ok(()); }
    let mut should_be_processed = generate_should_be_processed(&records);

    let mut ipv4: Option<Ipv4Addr> = None;
    if should_be_processed(RecordType::A) {
        ipv4 = Some(get_ip_address_by_resolver(addr_v4).await.ok_or(Error::ResolverError("no IPV4 found".to_owned()))?);
    }

    let mut ipv6: Option<Ipv6Addr> = None;
    if should_be_processed(RecordType::AAAA) {
        ipv6 = Some(get_ip_address_by_resolver(addr_v6).await.ok_or(Error::ResolverError("no IPv6 found".to_owned()))?);
    };

    match IPs::load(&APP_INFO, IP_KEY) {
        Ok(ips) => {
            match ips.ipv4 {
                None => {}
                Some(ip) => {
                    if ipv4.is_some() && ipv4.unwrap() == ip {
                        records.remove(&RecordType::A);
                    }
                }
            }
            match ips.ipv6 {
                None => {}
                Some(ip) => {
                    if ipv6.is_some() && ipv6.unwrap() == ip {
                        records.remove(&RecordType::AAAA);
                    }
                }
            }
        }
        Err(_) => (),
    };
    if records.is_empty() { return Ok(()); }
    should_be_processed = generate_should_be_processed(&records);


    let authentication_data_list =
        AuthenticationDataList::load(&APP_INFO, AUTH_KEY)?;

    let vendor_to_handler: HashMap<AuthenticationDataDiscriminants, Box<dyn UpdateHandler>> =
        authentication_data_list.into_iter()
            .map(create_handlers)
            .collect();

    for service in dns_entries {
        let service_discriminant = AuthenticationDataDiscriminants::from_str(service.service_name.as_str()).unwrap();
        for dns_entry in service.specifications {
            for host in dns_entry.specifications {
                for record in host.specifications {
                    if !should_be_processed(record.record_type) { continue; }

                    let handler = vendor_to_handler.get(&service_discriminant).unwrap();
                    match record.record_type {
                        RecordType::A =>
                            handler.update_ipv4_record(DnsRecord {
                                domain: dns_entry.domain_name.as_str(),
                                host: host.host_name.as_str(),
                                ttl: record.ttl,
                            }, ipv4.unwrap()).await?,
                        RecordType::AAAA =>
                            handler.update_ipv6_record(DnsRecord {
                                domain: dns_entry.domain_name.as_str(),
                                host: host.host_name.as_str(),
                                ttl: record.ttl,
                            }, ipv6.unwrap()).await?,
                    };
                }
            }
        }
    }

    let mut new_ips = IPs{ipv4:None, ipv6:None};
    if should_be_processed(RecordType::A){
        new_ips.ipv4 = Some(ipv4.unwrap())
    }
    if should_be_processed(RecordType::AAAA){
        new_ips.ipv6 = Some(ipv6.unwrap())
    }
    new_ips.save(&APP_INFO, IP_KEY)?;
    Ok(())
}