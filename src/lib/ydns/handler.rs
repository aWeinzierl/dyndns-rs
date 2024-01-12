use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::panic::panic_any;

use reqwest::StatusCode;

use crate::update_handler::UpdateHandler;
use crate::RecordSpecificationV6Adapter;

use super::{AuthenticationData, SpecificationV4, SpecificationV6};

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
                Err(super::Error::Request(format!(
                    "Request failed with Code {status:?}: {text}"
                )))
            }
        }
    }
}

impl UpdateHandler<AuthenticationData, SpecificationV4, SpecificationV6> for Handler {
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

    async fn update_ipv4_record(
        &self,
        _specification: &SpecificationV4,
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
        specification: &RecordSpecificationV6Adapter<SpecificationV6>,
        domain: &str,
        host: &str,
        mut ip:Ipv6Addr,
    ) -> Result<(), crate::Error> {
        if let Some(interface_id) = specification.custom_interface_id{
            replace_interface_id(&mut ip, interface_id);
        }
        self.update_ip_address(domain, host, &IpAddr::V6(ip))
            .await?;
        Ok(())
    }
}

fn replace_interface_id(ip: &mut Ipv6Addr, interface_id: Ipv6Addr) {
    let interface_id = interface_id.segments();
    let suffix = interface_id.rsplit_array_ref::<4>().1;
    let mut ipv6_segments = ip.segments();
    ipv6_segments[4..].copy_from_slice(suffix);
    *ip = Ipv6Addr::from(ipv6_segments);
}
