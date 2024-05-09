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

/// RacPropertyMappingRequest : RACPropertyMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RacPropertyMappingRequest {
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(rename = "managed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub managed: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "static_settings")]
    pub static_settings: std::collections::HashMap<String, serde_json::Value>,
}

impl RacPropertyMappingRequest {
    /// RACPropertyMapping Serializer
    pub fn new(name: String, static_settings: std::collections::HashMap<String, serde_json::Value>) -> RacPropertyMappingRequest {
        RacPropertyMappingRequest {
            managed: None,
            name,
            expression: None,
            static_settings,
        }
    }
}
