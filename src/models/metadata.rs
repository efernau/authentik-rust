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

/// Metadata : Serializer for blueprint metadata
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "labels")]
    pub labels: std::collections::HashMap<String, serde_json::Value>,
}

impl Metadata {
    /// Serializer for blueprint metadata
    pub fn new(name: String, labels: std::collections::HashMap<String, serde_json::Value>) -> Metadata {
        Metadata {
            name,
            labels,
        }
    }
}

