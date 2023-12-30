#![feature(async_closure)]

mod dns_record_list;
mod retry_handler;

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use dyndns_rs::*;
use futures_retry::FutureRetry;
use preferences::{AppInfo, Preferences};
use public_ip::{addr_v4, addr_v6};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumString, IntoStaticStr};

use dns_record_list::{DnsRecordList, DomainSpecifications, ServiceSpecificationsDiscriminants};
use retry_handler::RetryHandler;

fn retry_handler() -> RetryHandler {
    RetryHandler::new(3, 100)
}

type AuthenticationDataList = Vec<AuthenticationData>;

#[derive(Serialize, Deserialize, Debug, EnumDiscriminants, IntoStaticStr)]
#[strum_discriminants(derive(EnumString, Hash))]
enum AuthenticationData {
    GoDaddy(godaddy::AuthenticationData),
    YDns(ydns::AuthenticationData),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct IPs {
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}

const APP_INFO: AppInfo = AppInfo {
    name: "DynDns Service",
    author: "Andreas Weinzierl",
};
const AUTH_KEY: &str = "authentication";
const DNS_ENTRIES_KEY: &str = "dns-entries";
const IP_KEY: &str = "ips";

async fn get_ip_address_by_resolver<Resolver, AddrFuture, AddrType>(
    resolve: Resolver,
) -> Option<AddrType>
where
    Resolver: Fn() -> AddrFuture + Clone + Send + 'static,
    AddrFuture: Future<Output = Option<AddrType>> + Send + 'static,
{
    let ip = FutureRetry::new(async || resolve().await.ok_or(()), retry_handler())
        .await
        .ok()?
        .0;
    Some(ip)
}

fn collect_record_types_domain<AuthData, Specification, Handler>(
    set: &mut HashSet<RecordType>,
    specifications: &Vec<DomainSpecifications<Specification>>,
) where
    Handler: UpdateHandler<AuthData, Specification>,
{
    for specs in specifications {
        for specs in &specs.specifications {
            for specification in &specs.specifications {
                set.insert(Handler::record_type(specification));
            }
        }
    }
}

fn collect_record_types(dns_record_list: &DnsRecordList) -> HashSet<RecordType> {
    let mut set: HashSet<RecordType> = HashSet::new();
    for service in dns_record_list {
        match service {
            dns_record_list::ServiceSpecifications::GoDaddy(specs) => {
                collect_record_types_domain::<_, _, godaddy::Handler>(&mut set, specs)
            }
            dns_record_list::ServiceSpecifications::YDns(specs) => {
                collect_record_types_domain::<_, _, ydns::Handler>(&mut set, specs)
            }
        };
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
        _ => panic!("Got three different record types, although only 2 should exist"),
    }
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let dns_entries = DnsRecordList::load(&APP_INFO, DNS_ENTRIES_KEY)?;

    let mut records = collect_record_types(&dns_entries);
    if records.is_empty() {
        return Ok(());
    }
    let should_be_processed = generate_should_be_processed(&records);
    let mut ipv4: Option<Ipv4Addr> = None;
    if should_be_processed(RecordType::A) {
        ipv4 = Some(
            get_ip_address_by_resolver(addr_v4)
                .await
                .ok_or(Error::ResolverError("no IPV4 found".to_owned()))?,
        );
    }

    let mut ipv6: Option<Ipv6Addr> = None;
    if should_be_processed(RecordType::AAAA) {
        ipv6 = Some(
            get_ip_address_by_resolver(addr_v6)
                .await
                .ok_or(Error::ResolverError("no IPv6 found".to_owned()))?,
        );
    };

    let mut old_ips = IPs::load(&APP_INFO, IP_KEY).unwrap_or_default(); 
    match old_ips.ipv4 {
        None => old_ips.ipv4 = ipv4,
        Some(ip) => {
            if ipv4.is_some() && ipv4.unwrap() == ip {
                records.remove(&RecordType::A);
            } else {
                old_ips.ipv4 = ipv4;
            }
        }
    }
    match old_ips.ipv6 {
        None => old_ips.ipv6 = ipv6,
        Some(ip) => {
            if ipv6.is_some() && ipv6.unwrap() == ip {
                records.remove(&RecordType::AAAA);
            } else {
                old_ips.ipv6 = ipv6;
            }
        }
    }
    let new_ips = old_ips;

    if records.is_empty() {
        return Ok(());
    }
    let should_be_processed = generate_should_be_processed(&records);

    let authentication_data_list = AuthenticationDataList::load(&APP_INFO, AUTH_KEY)?;

    let service_to_auth_data: HashMap<ServiceSpecificationsDiscriminants, AuthenticationData> =
        authentication_data_list
            .into_iter()
            .map(|auth_data| {
                let str: &str = (&auth_data).into();
                let service_enum: ServiceSpecificationsDiscriminants =
                    ServiceSpecificationsDiscriminants::from_str(str)
                        .expect("Each Authentification Method must have an associated Service");
                (service_enum, auth_data)
            })
            .collect();

    for service in dns_entries {
        let service_discriminant = (&service).into();
        let auth_data =
            service_to_auth_data
                .get(&service_discriminant)
                .ok_or(Error::AuthenticationError(format!(
                    "No authentication data provided for {service_discriminant:?}."
                )))?;
        match service {
            dns_record_list::ServiceSpecifications::GoDaddy(specifications) => {
                let AuthenticationData::GoDaddy(auth_data) = auth_data else {
                    unreachable!()
                };
                let handler = godaddy::Handler::new(auth_data);
                handle_domains_by_service(
                    handler,
                    specifications,
                    &should_be_processed,
                    ipv4,
                    ipv6,
                )
                .await?;
            }
            dns_record_list::ServiceSpecifications::YDns(specifications) => {
                let AuthenticationData::YDns(auth_data) = auth_data else {
                    unreachable!()
                };
                let handler = ydns::Handler::new(auth_data);
                handle_domains_by_service(
                    handler,
                    specifications,
                    &should_be_processed,
                    ipv4,
                    ipv6,
                )
                .await?;
            }
        };
    }
    new_ips.save(&APP_INFO, IP_KEY)?;
    Ok(())
}

async fn handle_domains_by_service<'a, AuthData, RecordSpecification, Handler>(
    handler: Handler,
    specifications: impl IntoIterator<Item = DomainSpecifications<RecordSpecification>>,
    should_be_processed: &dyn Fn(RecordType) -> bool,
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
) -> Result<(), error::Error>
where
    Handler: UpdateHandler<AuthData, RecordSpecification>,
{
    for domain in specifications {
        for host in domain.specifications {
            for specification in host.specifications {
                let record_type = Handler::record_type(&specification);
                if !should_be_processed(record_type) {
                    continue;
                }
                match record_type {
                    RecordType::A => {
                        handler
                            .update_ipv4_record(
                                &specification,
                                &domain.domain_name,
                                &host.host_name,
                                ipv4.unwrap(),
                            )
                            .await?
                    }
                    RecordType::AAAA => {
                        handler
                            .update_ipv6_record(
                                &specification,
                                &domain.domain_name,
                                &host.host_name,
                                ipv6.unwrap(),
                            )
                            .await?
                    }
                }
            }
        }
    }
    Ok(())
}
