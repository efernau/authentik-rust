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

/// ServiceConnectionState : Serializer for Service connection state
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceConnectionState {
    #[serde(rename = "healthy")]
    pub healthy: bool,
    #[serde(rename = "version")]
    pub version: String,
}

impl ServiceConnectionState {
    /// Serializer for Service connection state
    pub fn new(healthy: bool, version: String) -> ServiceConnectionState {
        ServiceConnectionState {
            healthy,
            version,
        }
    }
}

