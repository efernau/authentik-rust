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

/// Event : Event Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<serde_json::Value>>,
    #[serde(rename = "action")]
    pub action: models::EventActions,
    #[serde(rename = "app")]
    pub app: String,
    #[serde(rename = "context", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub context: Option<Option<serde_json::Value>>,
    #[serde(rename = "client_ip", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<Option<String>>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "brand", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub brand: Option<Option<serde_json::Value>>,
}

impl Event {
    /// Event Serializer
    pub fn new(pk: uuid::Uuid, action: models::EventActions, app: String, created: String) -> Event {
        Event {
            pk,
            user: None,
            action,
            app,
            context: None,
            client_ip: None,
            created,
            expires: None,
            brand: None,
        }
    }
}
