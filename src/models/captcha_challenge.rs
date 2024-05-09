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

/// CaptchaChallenge : Site public key
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptchaChallenge {
    #[serde(rename = "type")]
    pub r#type: models::ChallengeChoices,
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<Box<models::ContextualFlowInfo>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "pending_user")]
    pub pending_user: String,
    #[serde(rename = "pending_user_avatar")]
    pub pending_user_avatar: String,
    #[serde(rename = "site_key")]
    pub site_key: String,
    #[serde(rename = "js_url")]
    pub js_url: String,
}

impl CaptchaChallenge {
    /// Site public key
    pub fn new(r#type: models::ChallengeChoices, pending_user: String, pending_user_avatar: String, site_key: String, js_url: String) -> CaptchaChallenge {
        CaptchaChallenge {
            r#type,
            flow_info: None,
            component: None,
            response_errors: None,
            pending_user,
            pending_user_avatar,
            site_key,
            js_url,
        }
    }
}
