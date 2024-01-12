use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumString, IntoStaticStr};

use dyndns_rs::{godaddy, ydns};

pub type AuthenticationDataList = Vec<AuthenticationData>;

#[derive(Serialize, Deserialize, Debug, EnumDiscriminants, IntoStaticStr)]
#[strum_discriminants(derive(EnumString, Hash))]
pub enum AuthenticationData {
    GoDaddy(godaddy::AuthenticationData),
    YDns(ydns::AuthenticationData),
}
