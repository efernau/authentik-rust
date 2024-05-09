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

/// Domain : Domain Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "is_primary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(rename = "tenant")]
    pub tenant: uuid::Uuid,
}

impl Domain {
    /// Domain Serializer
    pub fn new(id: i32, domain: String, tenant: uuid::Uuid) -> Domain {
        Domain {
            id,
            domain,
            is_primary: None,
            tenant,
        }
    }
}
