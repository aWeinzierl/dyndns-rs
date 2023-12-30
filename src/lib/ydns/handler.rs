use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::panic::panic_any;

use reqwest::StatusCode;

use crate::update_handler::UpdateHandler;
use crate::RecordType;

use super::{AuthenticationData, RecordSpecification};

pub struct Handler {
    http_client: reqwest::Client,
    user: String,
    password: String,
}

impl Handler {
    async fn update_ip_address(
        &self,
        domain: &str,
        name: &str,
        ip: &IpAddr,
    ) -> Result<(), super::Error> {
        let host_name = match name {
            "@" => domain.to_owned(),
            _ => format!("{name}.{domain}"),
        };
        let response = self
            .http_client
            .get(&(format!("https://ydns.io/api/v1/update/?host={host_name}&ip={ip}")))
            .basic_auth(&self.user, Some(&self.password))
            .send()
            .await?;
        let status = response.status();
        match status {
            StatusCode::OK => Ok(()),
            _ => {
                let text = response.text().await?;
                Err(super::Error::Request(format!("Request failed with Code {status:?}: {text}")))
            }
        }
    }
}

impl UpdateHandler<AuthenticationData, RecordSpecification> for Handler {
    fn new(AuthenticationData { username, secret }: &AuthenticationData) -> Self {
        let client = match reqwest::Client::builder().build() {
            Err(e) => panic_any(e),
            Ok(c) => c,
        };

        let handler = Handler {
            http_client: client,
            user: username.clone(),
            password: secret.clone(),
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
    ) -> Result<(), crate::Error> {
        self.update_ip_address(domain, host, &IpAddr::V4(ip))
            .await?;
        Ok(())
    }
    async fn update_ipv6_record(
        &self,
        _specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv6Addr,
    ) -> Result<(), crate::Error> {
        self.update_ip_address(domain, host, &IpAddr::V6(ip))
            .await?;
        Ok(())
    }
}
