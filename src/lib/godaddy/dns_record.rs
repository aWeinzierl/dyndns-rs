use serde::{Deserialize, Serialize};

use crate::RecordType;

use super::dns_data::DnsData;

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsRecord<'a> {
    pub name: &'a str,
    #[serde(rename = "type")]
    pub type_: RecordType,
    #[serde(rename = "data")]
    pub data: DnsData,
    pub ttl: u32,
}

impl DnsRecord<'_> {
    pub fn new(name: &str, type_: RecordType, data: DnsData, ttl: u32) -> DnsRecord {
        DnsRecord {
            name,
            type_,
            data,
            ttl,
        }
    }
}
