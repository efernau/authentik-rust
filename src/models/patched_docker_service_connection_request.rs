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

/// PatchedDockerServiceConnectionRequest : DockerServiceConnection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedDockerServiceConnectionRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If enabled, use the local connection. Required Docker socket/Kubernetes Integration
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    /// Can be in the format of 'unix://<path>' when connecting to a local docker daemon, or 'https://<hostname>:2376' when connecting to a remote system.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// CA which the endpoint's Certificate is verified against. Can be left empty for no validation.
    #[serde(rename = "tls_verification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tls_verification: Option<Option<uuid::Uuid>>,
    /// Certificate/Key used for authentication. Can be left empty for no authentication.
    #[serde(rename = "tls_authentication", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tls_authentication: Option<Option<uuid::Uuid>>,
}

impl PatchedDockerServiceConnectionRequest {
    /// DockerServiceConnection Serializer
    pub fn new() -> PatchedDockerServiceConnectionRequest {
        PatchedDockerServiceConnectionRequest {
            name: None,
            local: None,
            url: None,
            tls_verification: None,
            tls_authentication: None,
        }
    }
}
