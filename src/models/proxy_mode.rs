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

/// ProxyMode : * `proxy` - Proxy * `forward_single` - Forward Single * `forward_domain` - Forward Domain
/// * `proxy` - Proxy * `forward_single` - Forward Single * `forward_domain` - Forward Domain
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProxyMode {
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "forward_single")]
    ForwardSingle,
    #[serde(rename = "forward_domain")]
    ForwardDomain,

}

impl ToString for ProxyMode {
    fn to_string(&self) -> String {
        match self {
            Self::Proxy => String::from("proxy"),
            Self::ForwardSingle => String::from("forward_single"),
            Self::ForwardDomain => String::from("forward_domain"),
        }
    }
}

impl Default for ProxyMode {
    fn default() -> ProxyMode {
        Self::Proxy
    }
}

