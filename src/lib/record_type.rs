use std::net::Ipv6Addr;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use ux2::u6;

#[derive(Serialize, Deserialize, Display, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum RecordType {
    A,
    AAAA,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomInterfaceId {
    pub prefix_length: u6,
    pub interface_id: Ipv6Addr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecificationV6Adapter<RecordSpecificationV6> {
    pub record_specification: RecordSpecificationV6,
    pub custom_interface_id: Option<CustomInterfaceId>,
}
