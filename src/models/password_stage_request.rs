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

/// PasswordStageRequest : PasswordStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Selection of backends to test the password against.
    #[serde(rename = "backends")]
    pub backends: Vec<models::BackendsEnum>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(rename = "configure_flow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    /// How many attempts a user has before the flow is canceled. To lock the user out, use a reputation policy and a user_write stage.
    #[serde(rename = "failed_attempts_before_cancel", skip_serializing_if = "Option::is_none")]
    pub failed_attempts_before_cancel: Option<i32>,
}

impl PasswordStageRequest {
    /// PasswordStage Serializer
    pub fn new(name: String, backends: Vec<models::BackendsEnum>) -> PasswordStageRequest {
        PasswordStageRequest {
            name,
            flow_set: None,
            backends,
            configure_flow: None,
            failed_attempts_before_cancel: None,
        }
    }
}

