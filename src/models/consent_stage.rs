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

/// ConsentStage : ConsentStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentStage {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Get object type so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSet>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::ConsentStageModeEnum>,
    /// Offset after which consent expires. (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "consent_expire_in", skip_serializing_if = "Option::is_none")]
    pub consent_expire_in: Option<String>,
}

impl ConsentStage {
    /// ConsentStage Serializer
    pub fn new(pk: uuid::Uuid, name: String, component: String, verbose_name: String, verbose_name_plural: String, meta_model_name: String) -> ConsentStage {
        ConsentStage {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            flow_set: None,
            mode: None,
            consent_expire_in: None,
        }
    }
}

