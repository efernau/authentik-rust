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

/// NotificationRequest : Notification Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequest {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<models::EventRequest>>,
    #[serde(rename = "seen", skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
}

impl NotificationRequest {
    /// Notification Serializer
    pub fn new() -> NotificationRequest {
        NotificationRequest {
            event: None,
            seen: None,
        }
    }
}
