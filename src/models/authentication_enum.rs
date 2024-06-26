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

/// AuthenticationEnum : * `none` - None * `require_authenticated` - Require Authenticated * `require_unauthenticated` - Require Unauthenticated * `require_superuser` - Require Superuser * `require_outpost` - Require Outpost
/// * `none` - None * `require_authenticated` - Require Authenticated * `require_unauthenticated` - Require Unauthenticated * `require_superuser` - Require Superuser * `require_outpost` - Require Outpost
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticationEnum {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "require_authenticated")]
    RequireAuthenticated,
    #[serde(rename = "require_unauthenticated")]
    RequireUnauthenticated,
    #[serde(rename = "require_superuser")]
    RequireSuperuser,
    #[serde(rename = "require_outpost")]
    RequireOutpost,

}

impl ToString for AuthenticationEnum {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::RequireAuthenticated => String::from("require_authenticated"),
            Self::RequireUnauthenticated => String::from("require_unauthenticated"),
            Self::RequireSuperuser => String::from("require_superuser"),
            Self::RequireOutpost => String::from("require_outpost"),
        }
    }
}

impl Default for AuthenticationEnum {
    fn default() -> AuthenticationEnum {
        Self::None
    }
}

