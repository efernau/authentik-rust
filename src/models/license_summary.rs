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

/// LicenseSummary : Serializer for license status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseSummary {
    #[serde(rename = "internal_users")]
    pub internal_users: i32,
    #[serde(rename = "external_users")]
    pub external_users: i32,
    #[serde(rename = "valid")]
    pub valid: bool,
    #[serde(rename = "show_admin_warning")]
    pub show_admin_warning: bool,
    #[serde(rename = "show_user_warning")]
    pub show_user_warning: bool,
    #[serde(rename = "read_only")]
    pub read_only: bool,
    #[serde(rename = "latest_valid")]
    pub latest_valid: String,
    #[serde(rename = "has_license")]
    pub has_license: bool,
}

impl LicenseSummary {
    /// Serializer for license status
    pub fn new(internal_users: i32, external_users: i32, valid: bool, show_admin_warning: bool, show_user_warning: bool, read_only: bool, latest_valid: String, has_license: bool) -> LicenseSummary {
        LicenseSummary {
            internal_users,
            external_users,
            valid,
            show_admin_warning,
            show_user_warning,
            read_only,
            latest_valid,
            has_license,
        }
    }
}

