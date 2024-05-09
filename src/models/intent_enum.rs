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

/// IntentEnum : * `verification` - Intent Verification * `api` - Intent Api * `recovery` - Intent Recovery * `app_password` - Intent App Password
/// * `verification` - Intent Verification * `api` - Intent Api * `recovery` - Intent Recovery * `app_password` - Intent App Password
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntentEnum {
    #[serde(rename = "verification")]
    Verification,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "app_password")]
    AppPassword,

}

impl ToString for IntentEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Verification => String::from("verification"),
            Self::Api => String::from("api"),
            Self::Recovery => String::from("recovery"),
            Self::AppPassword => String::from("app_password"),
        }
    }
}

impl Default for IntentEnum {
    fn default() -> IntentEnum {
        Self::Verification
    }
}
