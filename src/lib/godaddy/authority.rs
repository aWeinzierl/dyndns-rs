use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::panic::panic_any;

use crate::RecordType;

use super::dns_data::DnsData;
use super::dns_record::DnsRecord;
use super::Error;

pub struct Authority {
    api_url: url::Url,
    http_client: reqwest::Client,
}

impl Authority {
    pub fn new<'a>(api_key: &'a str, api_secret: &'a str, api_url: url::Url) -> Authority {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_value = match reqwest::header::HeaderValue::from_str(
            &("sso-key ".to_owned() + api_key + ":" + api_secret),
        ) {
            Err(e) => panic_any(e),
            Ok(h) => h,
        };
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);

        let client = match reqwest::Client::builder().default_headers(headers).build() {
            Err(e) => panic_any(e),
            Ok(c) => c,
        };

        Authority {
            api_url,
            http_client: client,
        }
    }

    pub async fn update_ipv4_address(
        &self,
        domain: &str,
        name: &str,
        ip: &Ipv4Addr,
        ttl: u32,
    ) -> Result<(), Error> {
        self.update_ip_address(domain, name, &IpAddr::V4(*ip), ttl, RecordType::A)
            .await
    }

    pub async fn update_ipv6_address(
        &self,
        domain: &str,
        name: &str,
        ip: &Ipv6Addr,
        ttl: u32,
    ) -> Result<(), Error> {
        self.update_ip_address(domain, name, &IpAddr::V6(*ip), ttl, RecordType::AAAA)
            .await
    }

    async fn update_ip_address(
        &self,
        domain: &str,
        name: &str,
        ip: &IpAddr,
        ttl: u32,
        record_type: RecordType,
    ) -> Result<(), Error> {
        let _ =
            self.http_client
                .put(
                    &(self.api_url.to_string()
                        + "/v1/domains/"
                        + domain
                        + "\
                    /records/"
                        + &record_type.to_string()
                        + "\
                    /" + &name
                        + ""),
                )
                .json(&[DnsRecord::new(
                    name,
                    record_type,
                    DnsData::IpAddress(*ip),
                    ttl,
                )])
                .send()
                .await?
                .text()
                .await?;

        Ok(())
    }

    pub async fn get_ipv4_address(&self, domain: &str, name: &str) -> Result<Ipv4Addr, Error> {
        let result = self.get_address(domain, name, RecordType::A).await?;
        let ipv4 = if let IpAddr::V4(result) = result {
            result
        } else {
            unreachable!()
        };
        return Ok(ipv4);
    }

    pub async fn get_ipv6_address(&self, domain: &str, name: &str) -> Result<Ipv6Addr, Error> {
        let result = self.get_address(domain, name, RecordType::AAAA).await?;
        let ipv6 = if let IpAddr::V6(result) = result {
            result
        } else {
            unreachable!()
        };
        return Ok(ipv6);
    }

    async fn get_address(
        &self,
        domain: &str,
        name: &str,
        record_type: RecordType,
    ) -> Result<IpAddr, Error> {
        let response =
            self.http_client
                .get(
                    &(self.api_url.to_string()
                        + "/v1/domains/"
                        + domain
                        + "\
                    /records/"
                        + &record_type.to_string()
                        + "\
                    /" + &name
                        + ""),
                )
                .send()
                .await?
                .text()
                .await?;

        let record: Vec<DnsRecord> = serde_json::from_str(&response)?;
        assert!(record.len() <= 1, "Max one record is to be expected");
        let DnsData::IpAddress(address) = record[0].data;

        Ok(address)
    }
}
