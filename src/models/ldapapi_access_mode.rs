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

/// LdapapiAccessMode : * `direct` - Direct * `cached` - Cached
/// * `direct` - Direct * `cached` - Cached
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LdapapiAccessMode {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "cached")]
    Cached,

}

impl ToString for LdapapiAccessMode {
    fn to_string(&self) -> String {
        match self {
            Self::Direct => String::from("direct"),
            Self::Cached => String::from("cached"),
        }
    }
}

impl Default for LdapapiAccessMode {
    fn default() -> LdapapiAccessMode {
        Self::Direct
    }
}
