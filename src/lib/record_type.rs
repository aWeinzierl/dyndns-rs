use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Display, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum RecordType {
    A,
    AAAA,
}
