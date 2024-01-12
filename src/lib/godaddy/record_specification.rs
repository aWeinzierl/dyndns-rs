use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecification {
    pub ttl: u32,
}

pub type SpecificationV4 = RecordSpecification;
pub type SpecificationV6 = RecordSpecification;
