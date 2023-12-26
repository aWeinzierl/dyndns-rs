use serde::{Deserialize, Serialize};

use godaddy::RecordType;

pub type DnsRecordList = Vec<ServiceSpecifications>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceSpecifications {
    pub service_name: String,
    pub specifications: Vec<DomainSpecifications>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainSpecifications {
    pub domain_name: String,
    pub specifications: Vec<HostSpecifications>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostSpecifications {
    pub host_name: String,
    pub specifications: Vec<RecordSpecifications>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecifications {
    pub record_type: RecordType,
    pub ttl: u32,
}