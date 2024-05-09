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

/// AuthenticatorDuoStageRequest : AuthenticatorDuoStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorDuoStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(rename = "configure_flow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "api_hostname")]
    pub api_hostname: String,
    #[serde(rename = "admin_integration_key", skip_serializing_if = "Option::is_none")]
    pub admin_integration_key: Option<String>,
    #[serde(rename = "admin_secret_key", skip_serializing_if = "Option::is_none")]
    pub admin_secret_key: Option<String>,
}

impl AuthenticatorDuoStageRequest {
    /// AuthenticatorDuoStage Serializer
    pub fn new(name: String, client_id: String, client_secret: String, api_hostname: String) -> AuthenticatorDuoStageRequest {
        AuthenticatorDuoStageRequest {
            name,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            client_id,
            client_secret,
            api_hostname,
            admin_integration_key: None,
            admin_secret_key: None,
        }
    }
}
