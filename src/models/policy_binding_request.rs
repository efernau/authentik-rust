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

/// PolicyBindingRequest : PolicyBinding Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyBindingRequest {
    #[serde(rename = "policy", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Option<uuid::Uuid>>,
    #[serde(rename = "group", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<i32>>,
    #[serde(rename = "target")]
    pub target: uuid::Uuid,
    /// Negates the outcome of the policy. Messages are unaffected.
    #[serde(rename = "negate", skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "order")]
    pub order: i32,
    /// Timeout after which Policy execution is terminated.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Result if the Policy execution fails.
    #[serde(rename = "failure_result", skip_serializing_if = "Option::is_none")]
    pub failure_result: Option<bool>,
}

impl PolicyBindingRequest {
    /// PolicyBinding Serializer
    pub fn new(target: uuid::Uuid, order: i32) -> PolicyBindingRequest {
        PolicyBindingRequest {
            policy: None,
            group: None,
            user: None,
            target,
            negate: None,
            enabled: None,
            order,
            timeout: None,
            failure_result: None,
        }
    }
}

