use serde::{Deserialize, Serialize};

use crate::RecordType;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSpecification {
    pub record_type: RecordType,
}
