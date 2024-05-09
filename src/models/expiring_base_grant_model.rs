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

/// ExpiringBaseGrantModel : Serializer for BaseGrantModel and ExpiringBaseGrant
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpiringBaseGrantModel {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "provider")]
    pub provider: Box<models::OAuth2Provider>,
    #[serde(rename = "user")]
    pub user: Box<models::User>,
    /// Check if token is expired yet.
    #[serde(rename = "is_expired")]
    pub is_expired: bool,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "scope")]
    pub scope: Vec<String>,
}

impl ExpiringBaseGrantModel {
    /// Serializer for BaseGrantModel and ExpiringBaseGrant
    pub fn new(pk: i32, provider: models::OAuth2Provider, user: models::User, is_expired: bool, scope: Vec<String>) -> ExpiringBaseGrantModel {
        ExpiringBaseGrantModel {
            pk,
            provider: Box::new(provider),
            user: Box::new(user),
            is_expired,
            expires: None,
            scope,
        }
    }
}
