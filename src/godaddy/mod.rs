mod authority;
mod dns_data;
mod dns_record;
mod error;
mod predefined_url;
mod record_type;

pub use authority::Authority;
use dns_data::DnsData;
use dns_record::DnsRecord;
pub use error::Error;
pub use predefined_url::PredefinedUrl;
pub use record_type::RecordType;
