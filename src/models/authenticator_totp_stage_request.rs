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

/// AuthenticatorTotpStageRequest : AuthenticatorTOTPStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorTotpStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(rename = "configure_flow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "digits")]
    pub digits: models::DigitsEnum,
}

impl AuthenticatorTotpStageRequest {
    /// AuthenticatorTOTPStage Serializer
    pub fn new(name: String, digits: models::DigitsEnum) -> AuthenticatorTotpStageRequest {
        AuthenticatorTotpStageRequest {
            name,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            digits,
        }
    }
}

