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

/// SpBindingEnum : * `redirect` - Redirect * `post` - Post
/// * `redirect` - Redirect * `post` - Post
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpBindingEnum {
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "post")]
    Post,

}

impl ToString for SpBindingEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Redirect => String::from("redirect"),
            Self::Post => String::from("post"),
        }
    }
}

impl Default for SpBindingEnum {
    fn default() -> SpBindingEnum {
        Self::Redirect
    }
}
