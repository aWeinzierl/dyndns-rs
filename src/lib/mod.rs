#![feature(split_array)]

pub mod godaddy;
pub mod ydns;

mod update_handler;

pub mod error;
pub mod record_type;

pub use error::Error;
pub use record_type::{RecordSpecificationV6Adapter, RecordType};
pub use update_handler::UpdateHandler;
