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

/// PatchedUserDeleteStageRequest : UserDeleteStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserDeleteStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
}

impl PatchedUserDeleteStageRequest {
    /// UserDeleteStage Serializer
    pub fn new() -> PatchedUserDeleteStageRequest {
        PatchedUserDeleteStageRequest {
            name: None,
            flow_set: None,
        }
    }
}

