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

/// ClientTypeEnum : * `confidential` - Confidential * `public` - Public
/// * `confidential` - Confidential * `public` - Public
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientTypeEnum {
    #[serde(rename = "confidential")]
    Confidential,
    #[serde(rename = "public")]
    Public,

}

impl ToString for ClientTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Confidential => String::from("confidential"),
            Self::Public => String::from("public"),
        }
    }
}

impl Default for ClientTypeEnum {
    fn default() -> ClientTypeEnum {
        Self::Confidential
    }
}

