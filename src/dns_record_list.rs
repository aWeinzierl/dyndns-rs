use serde::{Deserialize, Serialize};

use godaddy::RecordType;

pub type DnsRecordList = Vec<ServiceSpecifications>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceSpecifications {
    pub service_name: String,
    pub specifications: Vec<DomainSpecifications<RecordSpecification>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainSpecifications<RecordSpecification> {
    pub domain_name: String,
    pub specifications: Vec<HostSpecifications<RecordSpecification>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostSpecifications<RecordSpecification> {
    pub host_name: String,
    pub specifications: Vec<RecordSpecification>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecification {
    pub record_type: RecordType,
    pub ttl: u32,
}