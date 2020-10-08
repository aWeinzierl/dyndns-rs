use serde::{Deserialize, Serialize};

use godaddy::PredefinedUrl;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoDaddyAuthenticationData {
    pub api_key: String,
    pub api_secret: String,
    pub api_url: ApiUrl,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiUrl {
    PredefinedUrl(PredefinedUrl),
    CustomUrl(url::Url),
}

impl From<ApiUrl> for url::Url {
    fn from(api_url: ApiUrl) -> Self {
        match api_url {
            ApiUrl::PredefinedUrl(url) => url.into(),
            ApiUrl::CustomUrl(url) => url,
        }
    }
}

impl From<PredefinedUrl> for ApiUrl {
    fn from(predefined_url: PredefinedUrl) -> Self {
        ApiUrl::PredefinedUrl(predefined_url)
    }
}

impl From<url::Url> for ApiUrl {
    fn from(url: url::Url) -> Self {
        ApiUrl::CustomUrl(url)
    }
}
