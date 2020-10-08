use std::net::{Ipv4Addr, Ipv6Addr};

use async_trait::async_trait;

use crate::error::Error;

pub trait CreatableUpdateHandler<DataType> {
    fn new(auth_data: DataType) -> Box<dyn UpdateHandler>;
}

pub struct DnsRecord<'a> {
    pub domain: &'a str,
    pub host: &'a str,
    pub ttl: u32,
}

#[async_trait]
pub trait UpdateHandler {
    async fn update_ipv4_record(&self, dns_record: DnsRecord<'_>, ip: Ipv4Addr) -> Result<(), Error>;
    async fn update_ipv6_record(&self, dns_record: DnsRecord<'_>, ip: Ipv6Addr) -> Result<(), Error>;
}