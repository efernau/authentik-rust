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

/// SystemTaskStatusEnum : * `unknown` - UNKNOWN * `successful` - SUCCESSFUL * `warning` - WARNING * `error` - ERROR
/// * `unknown` - UNKNOWN * `successful` - SUCCESSFUL * `warning` - WARNING * `error` - ERROR
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemTaskStatusEnum {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,

}

impl ToString for SystemTaskStatusEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("unknown"),
            Self::Successful => String::from("successful"),
            Self::Warning => String::from("warning"),
            Self::Error => String::from("error"),
        }
    }
}

impl Default for SystemTaskStatusEnum {
    fn default() -> SystemTaskStatusEnum {
        Self::Unknown
    }
}

