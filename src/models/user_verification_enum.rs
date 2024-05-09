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

/// UserVerificationEnum : * `required` - Required * `preferred` - Preferred * `discouraged` - Discouraged
/// * `required` - Required * `preferred` - Preferred * `discouraged` - Discouraged
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserVerificationEnum {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "discouraged")]
    Discouraged,

}

impl ToString for UserVerificationEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Required => String::from("required"),
            Self::Preferred => String::from("preferred"),
            Self::Discouraged => String::from("discouraged"),
        }
    }
}

impl Default for UserVerificationEnum {
    fn default() -> UserVerificationEnum {
        Self::Required
    }
}
