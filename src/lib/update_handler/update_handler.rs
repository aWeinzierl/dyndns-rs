use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{Error, RecordSpecificationV6Adapter};

pub trait UpdateHandler<AuthenticationData, RecordSpecificationV4, RecordSpecificationV6> {
    fn new(auth_data: &AuthenticationData) -> Self;
    async fn update_ipv4_record(
        &self,
        specification: &RecordSpecificationV4,
        domain: &str,
        host: &str,
        ip: Ipv4Addr,
    ) -> Result<(), Error>;
    async fn update_ipv6_record(
        &self,
        specification: &RecordSpecificationV6Adapter<RecordSpecificationV6>,
        domain: &str,
        host: &str,
        ip: Ipv6Addr,
    ) -> Result<(), Error>;
}
