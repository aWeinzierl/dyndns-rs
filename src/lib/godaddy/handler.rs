use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{Error, RecordType, UpdateHandler};

use super::record_specification::RecordSpecification;
use super::{AuthenticationData, Authority};

pub struct Handler {
    authority: Authority,
}

impl UpdateHandler<AuthenticationData, RecordSpecification> for Handler {
    fn new(auth_data: &AuthenticationData) -> Handler {
        let handler = Handler {
            authority: Authority::new(
                auth_data.api_key.as_str(),
                auth_data.api_secret.as_str(),
                auth_data.api_url.clone().into(),
            ),
        };
        handler
    }

    fn record_type(specification: &RecordSpecification) -> RecordType {
        specification.record_type
    }

    async fn update_ipv6_record(
        &self,
        specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv6Addr,
    ) -> Result<(), Error> {
        self.authority
            .update_ipv6_address(domain, host, &ip, specification.ttl)
            .await?;
        Ok(())
    }
    async fn update_ipv4_record(
        &self,
        specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv4Addr,
    ) -> Result<(), Error> {
        self.authority
            .update_ipv4_address(domain, host, &ip, specification.ttl)
            .await?;
        Ok(())
    }
}
