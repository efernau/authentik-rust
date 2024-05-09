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

/// PolicyTestResult : result of a policy test
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyTestResult {
    #[serde(rename = "passing")]
    pub passing: bool,
    #[serde(rename = "messages")]
    pub messages: Vec<String>,
    #[serde(rename = "log_messages")]
    pub log_messages: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

impl PolicyTestResult {
    /// result of a policy test
    pub fn new(passing: bool, messages: Vec<String>, log_messages: Vec<std::collections::HashMap<String, serde_json::Value>>) -> PolicyTestResult {
        PolicyTestResult {
            passing,
            messages,
            log_messages,
        }
    }
}

