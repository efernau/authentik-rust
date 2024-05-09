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

/// Device : Serializer for Duo authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// Get type of device
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "confirmed")]
    pub confirmed: bool,
}

impl Device {
    /// Serializer for Duo authenticator devices
    pub fn new(verbose_name: String, verbose_name_plural: String, meta_model_name: String, pk: i32, name: String, r#type: String, confirmed: bool) -> Device {
        Device {
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            pk,
            name,
            r#type,
            confirmed,
        }
    }
}

