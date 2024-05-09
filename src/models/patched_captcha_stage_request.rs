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

/// PatchedCaptchaStageRequest : CaptchaStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedCaptchaStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Public key, acquired your captcha Provider.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Private key, acquired your captcha Provider.
    #[serde(rename = "private_key", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "js_url", skip_serializing_if = "Option::is_none")]
    pub js_url: Option<String>,
    #[serde(rename = "api_url", skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
}

impl PatchedCaptchaStageRequest {
    /// CaptchaStage Serializer
    pub fn new() -> PatchedCaptchaStageRequest {
        PatchedCaptchaStageRequest {
            name: None,
            flow_set: None,
            public_key: None,
            private_key: None,
            js_url: None,
            api_url: None,
        }
    }
}
