use serde::{Deserialize, Serialize};

use godaddy::RecordType;

pub type DnsRecordList = Vec<DomainSpecification>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainSpecification {
    pub domain_name: String,
    pub specifications: Vec<HostSpecification>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostSpecification {
    pub host_name: String,
    pub specifications: Vec<RecordSpecification>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecification {
    pub record_type: RecordType,
    pub ttl: u32,
}