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

/// InvalidResponseActionEnum : * `retry` - Retry * `restart` - Restart * `restart_with_context` - Restart With Context
/// * `retry` - Retry * `restart` - Restart * `restart_with_context` - Restart With Context
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvalidResponseActionEnum {
    #[serde(rename = "retry")]
    Retry,
    #[serde(rename = "restart")]
    Restart,
    #[serde(rename = "restart_with_context")]
    RestartWithContext,

}

impl ToString for InvalidResponseActionEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Retry => String::from("retry"),
            Self::Restart => String::from("restart"),
            Self::RestartWithContext => String::from("restart_with_context"),
        }
    }
}

impl Default for InvalidResponseActionEnum {
    fn default() -> InvalidResponseActionEnum {
        Self::Retry
    }
}

