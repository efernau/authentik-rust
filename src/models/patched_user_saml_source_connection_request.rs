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

/// PatchedUserSamlSourceConnectionRequest : SAML Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserSamlSourceConnectionRequest {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PatchedUserSamlSourceConnectionRequest {
    /// SAML Source Serializer
    pub fn new() -> PatchedUserSamlSourceConnectionRequest {
        PatchedUserSamlSourceConnectionRequest {
            user: None,
            identifier: None,
        }
    }
}
