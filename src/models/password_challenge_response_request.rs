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

/// PasswordChallengeResponseRequest : Password challenge response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "password")]
    pub password: String,
}

impl PasswordChallengeResponseRequest {
    /// Password challenge response
    pub fn new(password: String) -> PasswordChallengeResponseRequest {
        PasswordChallengeResponseRequest {
            component: None,
            password,
        }
    }
}

