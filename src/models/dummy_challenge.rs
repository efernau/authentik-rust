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

/// DummyChallenge : Dummy challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DummyChallenge {
    #[serde(rename = "type")]
    pub r#type: models::ChallengeChoices,
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<Box<models::ContextualFlowInfo>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
}

impl DummyChallenge {
    /// Dummy challenge
    pub fn new(r#type: models::ChallengeChoices) -> DummyChallenge {
        DummyChallenge {
            r#type,
            flow_info: None,
            component: None,
            response_errors: None,
        }
    }
}

