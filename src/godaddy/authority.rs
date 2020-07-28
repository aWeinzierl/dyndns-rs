use super::RecordType;

pub struct Authority<'a> {
    api_url: &'a str,
    api_key: &'a str,
    api_secret: &'a str,
}

pub mod api_url {
    pub const OTE: &str = "https://api.ote-godaddy.com";
    pub const PRODUCTION: &str = "https://api.godaddy.com";
}

impl Authority<'_> {
    pub fn new<'a>(api_key: &'a str, api_secret: &'a str, api_url: &'a str) -> Authority<'a> {
        Authority {
            api_url,
            api_key,
            api_secret,
        }
    }

    pub fn update_subdoomain(&self, subdomain: &str, record_type: RecordType ){
        print!("{}",record_type.to_string());
    }
}
