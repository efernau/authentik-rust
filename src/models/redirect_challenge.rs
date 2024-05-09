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

/// RedirectChallenge : Challenge type to redirect the client
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectChallenge {
    #[serde(rename = "type")]
    pub r#type: models::ChallengeChoices,
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<Box<models::ContextualFlowInfo>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "to")]
    pub to: String,
}

impl RedirectChallenge {
    /// Challenge type to redirect the client
    pub fn new(r#type: models::ChallengeChoices, to: String) -> RedirectChallenge {
        RedirectChallenge {
            r#type,
            flow_info: None,
            component: None,
            response_errors: None,
            to,
        }
    }
}
