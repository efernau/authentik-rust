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

/// PatchedLicenseRequest : License Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedLicenseRequest {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl PatchedLicenseRequest {
    /// License Serializer
    pub fn new() -> PatchedLicenseRequest {
        PatchedLicenseRequest {
            key: None,
        }
    }
}
