
use strum_macros::{EnumDiscriminants, EnumString};
use std::str::FromStr;

use godaddy::RecordType;
use serde::{Deserialize, Serialize};

use crate::{godaddy_handler, ydns};

pub type DnsRecordList = Vec<ServiceSpecifications>;

#[derive(Serialize, Deserialize, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Hash))]
pub enum ServiceSpecifications{
    GoDaddy(Vec<DomainSpecifications<godaddy_handler::RecordSpecification>>),
    YDns(Vec<DomainSpecifications<ydns::RecordSpecification>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainSpecifications<RecordSpecification> {
    pub domain_name: String,
    pub specifications: Vec<HostSpecifications<RecordSpecification>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostSpecifications<RecordSpecification> {
    pub host_name: String,
    pub specifications: Vec<(RecordType, RecordSpecification)>,
}
