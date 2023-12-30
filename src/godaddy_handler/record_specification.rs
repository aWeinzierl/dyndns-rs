use godaddy::RecordType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecification {
    pub record_type: RecordType,
    pub ttl: u32,
}