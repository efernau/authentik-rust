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

/// Prompt : Prompt Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Name of the form field, also used to store the value
    #[serde(rename = "field_key")]
    pub field_key: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "type")]
    pub r#type: models::PromptTypeEnum,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Optionally provide a short hint that describes the expected input value. When creating a fixed choice field, enable interpreting as expression and return a list to return multiple choices.
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Optionally pre-fill the input with an initial value. When creating a fixed choice field, enable interpreting as expression and return a list to return multiple default choices.
    #[serde(rename = "initial_value", skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "promptstage_set", skip_serializing_if = "Option::is_none")]
    pub promptstage_set: Option<Vec<models::Stage>>,
    #[serde(rename = "sub_text", skip_serializing_if = "Option::is_none")]
    pub sub_text: Option<String>,
    #[serde(rename = "placeholder_expression", skip_serializing_if = "Option::is_none")]
    pub placeholder_expression: Option<bool>,
    #[serde(rename = "initial_value_expression", skip_serializing_if = "Option::is_none")]
    pub initial_value_expression: Option<bool>,
}

impl Prompt {
    /// Prompt Serializer
    pub fn new(pk: uuid::Uuid, name: String, field_key: String, label: String, r#type: models::PromptTypeEnum) -> Prompt {
        Prompt {
            pk,
            name,
            field_key,
            label,
            r#type,
            required: None,
            placeholder: None,
            initial_value: None,
            order: None,
            promptstage_set: None,
            sub_text: None,
            placeholder_expression: None,
            initial_value_expression: None,
        }
    }
}

