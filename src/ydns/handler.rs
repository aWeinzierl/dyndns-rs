use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};

use godaddy::{self, RecordType};

use crate::error::Error;
use crate::update_handler::UpdateHandler;

use super::{AuthenticationData, RecordSpecification};

use std::panic::panic_any;

pub struct Handler {
    http_client: reqwest::Client,
}

impl Handler{
    async fn update_ip_address(&self, domain: &str, name: &str, ip: &IpAddr, record_type: RecordType) -> Result<(), Error> {
        let _ = self.http_client
            .get(&(
                "URL here".to_owned() + "/v1/domains/" + domain + "/records/" + &record_type.to_string() + "/" + &name + ""))
            .send().await?
            .text().await?;
        Ok(())
    }

    pub async fn update_ipv4_address(&self, domain: &str, name: &str, ip: &Ipv4Addr)
        -> Result<(), Error> {
    self.update_ip_address(domain, name, &IpAddr::V4(*ip), RecordType::A)
    .await
    }

    pub async fn update_ipv6_address(&self, domain: &str, name: &str, ip: &Ipv6Addr)
        -> Result<(), Error> {
    self.update_ip_address(domain, name, &IpAddr::V6(*ip), RecordType::AAAA)
    .await
    }

}


impl UpdateHandler<AuthenticationData, RecordSpecification> for Handler {
    fn new(auth_data: &AuthenticationData) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_value = match reqwest::header::HeaderValue::from_str(
            &("sso-key ".to_owned() + &auth_data.username + ":" + &auth_data.secret)) {
            Err(e) => panic_any(e),
            Ok(h) => h
        };
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);
    
        let client = match reqwest::Client::builder()
            .default_headers(headers)
            .build() {
            Err(e) => panic_any(e),
            Ok(c) => c
        };
    
        let handler = Handler{
            http_client: client,
        };

        handler
    }

    fn record_type(&self, specification: &RecordSpecification) -> RecordType{
        specification.record_type
    }

    async fn update_ipv4_record(&self, specification: &RecordSpecification, domain: &str, host: &str, ip: Ipv4Addr) -> Result<(), Error> {
        self.update_ipv4_address(
            domain,
            host,
            &ip,
        ).await?;
        Ok(())
    }
    async fn update_ipv6_record(&self, specification: &RecordSpecification, domain: &str, host: &str, ip: Ipv6Addr) -> Result<(), Error> {
        self.update_ipv6_address(
            domain,
            host,
            &ip,
        ).await?;
        Ok(())
    }

}