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

/// ProviderEnum : * `twilio` - Twilio * `generic` - Generic
/// * `twilio` - Twilio * `generic` - Generic
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderEnum {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "generic")]
    Generic,

}

impl ToString for ProviderEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Twilio => String::from("twilio"),
            Self::Generic => String::from("generic"),
        }
    }
}

impl Default for ProviderEnum {
    fn default() -> ProviderEnum {
        Self::Twilio
    }
}

