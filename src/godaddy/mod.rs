pub use authority::Authority;
use dns_data::DnsData;
use dns_record::DnsRecord;
pub use error::Error;
pub use predefined_url::PredefinedUrl;
pub use record_type::RecordType;

mod error;
mod dns_record;
mod dns_data;
mod authority;
mod record_type;
mod predefined_url;