use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::panic::panic_any;

use crate::error::Error;
use crate::update_handler::UpdateHandler;
use crate::RecordType;

use super::{AuthenticationData, RecordSpecification};

pub struct Handler {
    http_client: reqwest::Client,
}

impl Handler {
    async fn update_ip_address(
        &self,
        domain: &str,
        name: &str,
        ip: &IpAddr,
    ) -> Result<(), Error> {
        let _ = self
            .http_client
            .get(
                &(format!("https://ydns.io/api/v1/update/?host={name}.{domain}&ip={ip}")),
            )
            .send()
            .await?
            .text()
            .await?;
        Ok(())
    }

}

impl UpdateHandler<AuthenticationData, RecordSpecification> for Handler {
    fn new(AuthenticationData{username,secret}: &AuthenticationData) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_value = match reqwest::header::HeaderValue::from_str(
            &format!("{username}:{secret}")
        ) {
            Err(e) => panic_any(e),
            Ok(h) => h,
        };
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);

        let client = match reqwest::Client::builder().default_headers(headers).build() {
            Err(e) => panic_any(e),
            Ok(c) => c,
        };

        let handler = Handler {
            http_client: client,
        };

        handler
    }

    fn record_type(specification: &RecordSpecification) -> RecordType {
        specification.record_type
    }

    async fn update_ipv4_record(
        &self,
        _specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv4Addr,
    ) -> Result<(), Error> {
        self.update_ip_address(domain, host, &IpAddr::V4(ip)).await?;
        Ok(())
    }
    async fn update_ipv6_record(
        &self,
        _specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv6Addr,
    ) -> Result<(), Error> {
        self.update_ip_address(domain, host, &IpAddr::V6(ip)).await?;
        Ok(())
    }
}
