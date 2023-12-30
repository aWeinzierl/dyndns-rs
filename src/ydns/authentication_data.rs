use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationData {
    pub username: String,
    pub secret: String,
}
