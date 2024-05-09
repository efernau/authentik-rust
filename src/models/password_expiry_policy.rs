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

/// PasswordExpiryPolicy : Password Expiry Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordExpiryPolicy {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    /// Get object component so that we know how to edit the object
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
    /// Return objects policy is bound to
    #[serde(rename = "bound_to")]
    pub bound_to: i32,
    #[serde(rename = "days")]
    pub days: i32,
    #[serde(rename = "deny_only", skip_serializing_if = "Option::is_none")]
    pub deny_only: Option<bool>,
}

impl PasswordExpiryPolicy {
    /// Password Expiry Policy Serializer
    pub fn new(pk: uuid::Uuid, name: String, component: String, verbose_name: String, verbose_name_plural: String, meta_model_name: String, bound_to: i32, days: i32) -> PasswordExpiryPolicy {
        PasswordExpiryPolicy {
            pk,
            name,
            execution_logging: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            bound_to,
            days,
            deny_only: None,
        }
    }
}

