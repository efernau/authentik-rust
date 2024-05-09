/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// OutpostTypeEnum : * `proxy` - Proxy * `ldap` - Ldap * `radius` - Radius * `rac` - Rac
/// * `proxy` - Proxy * `ldap` - Ldap * `radius` - Radius * `rac` - Rac
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutpostTypeEnum {
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "ldap")]
    Ldap,
    #[serde(rename = "radius")]
    Radius,
    #[serde(rename = "rac")]
    Rac,

}

impl ToString for OutpostTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Proxy => String::from("proxy"),
            Self::Ldap => String::from("ldap"),
            Self::Radius => String::from("radius"),
            Self::Rac => String::from("rac"),
        }
    }
}

impl Default for OutpostTypeEnum {
    fn default() -> OutpostTypeEnum {
        Self::Proxy
    }
}
