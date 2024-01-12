use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{Error, RecordSpecificationV6Adapter, UpdateHandler};

use super::record_specification::{SpecificationV4, SpecificationV6};
use super::{AuthenticationData, Authority};

pub struct Handler {
    authority: Authority,
}

impl UpdateHandler<AuthenticationData, SpecificationV4, SpecificationV6> for Handler {
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

    async fn update_ipv6_record(
        &self,
        specification: &RecordSpecificationV6Adapter<SpecificationV6>,
        domain: &str,
        host: &str,
        ip: Ipv6Addr,
    ) -> Result<(), Error> {
        if specification.custom_interface_id.is_some() {
            return Err(super::error::Error::NotImplemented(
                "No support for hard coded interface identifiers yet.",
            )
            .into());
        }
        self.authority
            .update_ipv6_address(domain, host, &ip, specification.record_specification.ttl)
            .await?;
        Ok(())
    }
    async fn update_ipv4_record(
        &self,
        specification: &SpecificationV4,
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
