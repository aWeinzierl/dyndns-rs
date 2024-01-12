use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumString};

use dyndns_rs::{godaddy, ydns, RecordSpecificationV6Adapter};

pub type DnsRecordList = Vec<ServiceSpecifications>;

#[derive(Serialize, Deserialize, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Hash))]
pub enum ServiceSpecifications {
    GoDaddy(Vec<DomainSpecifications<godaddy::RecordSpecification, godaddy::RecordSpecification>>),
    YDns(Vec<DomainSpecifications<ydns::SpecificationV4, ydns::SpecificationV6>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainSpecifications<RecordSpecificationV4, RecordSpecificationV6> {
    pub domain_name: String,
    pub specifications: Vec<HostSpecifications<RecordSpecificationV4, RecordSpecificationV6>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostSpecifications<RecordSpecificationV4, RecordSpecificationV6> {
    pub host_name: String,
    pub ipv4: Option<RecordSpecificationV4>,
    pub ipv6: Option<RecordSpecificationV6Adapter<RecordSpecificationV6>>,
}
