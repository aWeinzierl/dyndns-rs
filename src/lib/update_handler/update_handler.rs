use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{Error, RecordType};

pub trait UpdateHandler<AuthenticationData, RecordSpecification> {
    fn new(auth_data: &AuthenticationData) -> Self;
    async fn update_ipv4_record(
        &self,
        specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv4Addr,
    ) -> Result<(), Error>;
    async fn update_ipv6_record(
        &self,
        specification: &RecordSpecification,
        domain: &str,
        host: &str,
        ip: Ipv6Addr,
    ) -> Result<(), Error>;
    fn record_type(specification: &RecordSpecification) -> RecordType;
}
