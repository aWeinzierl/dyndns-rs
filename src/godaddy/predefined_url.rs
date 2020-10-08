use serde::{Deserialize, Serialize};

impl From<PredefinedUrl> for url::Url {
    fn from(predefined_url: PredefinedUrl) -> Self {
        let str = match predefined_url {
            PredefinedUrl::OTE => "https://api.ote-godaddy.com",
            PredefinedUrl::PRODUCTION => "https://api.godaddy.com",
        };

        url::Url::parse(str).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PredefinedUrl {
    OTE,
    PRODUCTION,
}