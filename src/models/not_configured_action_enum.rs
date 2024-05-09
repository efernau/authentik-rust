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

/// NotConfiguredActionEnum : * `skip` - Skip * `deny` - Deny * `configure` - Configure
/// * `skip` - Skip * `deny` - Deny * `configure` - Configure
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotConfiguredActionEnum {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "configure")]
    Configure,

}

impl ToString for NotConfiguredActionEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Skip => String::from("skip"),
            Self::Deny => String::from("deny"),
            Self::Configure => String::from("configure"),
        }
    }
}

impl Default for NotConfiguredActionEnum {
    fn default() -> NotConfiguredActionEnum {
        Self::Skip
    }
}

