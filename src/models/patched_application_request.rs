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

/// PatchedApplicationRequest : Application Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedApplicationRequest {
    /// Application's display Name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Internal application name, used in URLs.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "provider", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Option<i32>>,
    #[serde(rename = "backchannel_providers", skip_serializing_if = "Option::is_none")]
    pub backchannel_providers: Option<Vec<i32>>,
    /// Open launch URL in a new browser tab or window.
    #[serde(rename = "open_in_new_tab", skip_serializing_if = "Option::is_none")]
    pub open_in_new_tab: Option<bool>,
    #[serde(rename = "meta_launch_url", skip_serializing_if = "Option::is_none")]
    pub meta_launch_url: Option<String>,
    #[serde(rename = "meta_description", skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    #[serde(rename = "meta_publisher", skip_serializing_if = "Option::is_none")]
    pub meta_publisher: Option<String>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl PatchedApplicationRequest {
    /// Application Serializer
    pub fn new() -> PatchedApplicationRequest {
        PatchedApplicationRequest {
            name: None,
            slug: None,
            provider: None,
            backchannel_providers: None,
            open_in_new_tab: None,
            meta_launch_url: None,
            meta_description: None,
            meta_publisher: None,
            policy_engine_mode: None,
            group: None,
        }
    }
}
