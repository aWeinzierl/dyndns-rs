use std::net::{Ipv4Addr, Ipv6Addr};

use async_trait::async_trait;

use godaddy;

use crate::error::Error;
use crate::update_handler::{CreatableUpdateHandler, DnsRecord, UpdateHandler};

use super::GoDaddyAuthenticationData;

pub struct GodaddyHandler {
    authority: godaddy::Authority,
}


impl CreatableUpdateHandler<GoDaddyAuthenticationData> for GodaddyHandler {
    fn new(auth_data: GoDaddyAuthenticationData) -> Box<dyn UpdateHandler> {
        let handler = GodaddyHandler {
            authority: godaddy::Authority::new(
                auth_data.api_key.as_str(),
                auth_data.api_secret.as_str(),
                auth_data.api_url.into(),
            )
        };
        Box::new(handler)
    }
}

#[async_trait]
impl UpdateHandler for GodaddyHandler {
    async fn update_ipv4_record(&self, dns_record: DnsRecord<'_>, ip: Ipv4Addr) -> Result<(), Error> {
        self.authority.update_ipv4_address(
            dns_record.domain,
            dns_record.host,
            &ip,
            dns_record.ttl,
        ).await?;
        Ok(())
    }
    async fn update_ipv6_record(&self, dns_record: DnsRecord<'_>, ip: Ipv6Addr) -> Result<(), Error> {
        self.authority.update_ipv6_address(
            dns_record.domain,
            dns_record.host,
            &ip,
            dns_record.ttl,
        ).await?;
        Ok(())
    }
}