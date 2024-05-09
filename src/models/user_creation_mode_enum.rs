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

/// UserCreationModeEnum : * `never_create` - Never Create * `create_when_required` - Create When Required * `always_create` - Always Create
/// * `never_create` - Never Create * `create_when_required` - Create When Required * `always_create` - Always Create
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserCreationModeEnum {
    #[serde(rename = "never_create")]
    NeverCreate,
    #[serde(rename = "create_when_required")]
    CreateWhenRequired,
    #[serde(rename = "always_create")]
    AlwaysCreate,

}

impl ToString for UserCreationModeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::NeverCreate => String::from("never_create"),
            Self::CreateWhenRequired => String::from("create_when_required"),
            Self::AlwaysCreate => String::from("always_create"),
        }
    }
}

impl Default for UserCreationModeEnum {
    fn default() -> UserCreationModeEnum {
        Self::NeverCreate
    }
}
