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

/// ConnectionTokenRequest : ConnectionToken Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionTokenRequest {
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl ConnectionTokenRequest {
    /// ConnectionToken Serializer
    pub fn new(provider: i32) -> ConnectionTokenRequest {
        ConnectionTokenRequest {
            provider,
        }
    }
}

