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

/// Version : Get running and latest version.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    /// Get current version
    #[serde(rename = "version_current")]
    pub version_current: String,
    /// Get latest version from cache
    #[serde(rename = "version_latest")]
    pub version_latest: String,
    /// Get build hash, if version is not latest or released
    #[serde(rename = "build_hash")]
    pub build_hash: String,
    /// Check if we're running the latest version
    #[serde(rename = "outdated")]
    pub outdated: bool,
}

impl Version {
    /// Get running and latest version.
    pub fn new(version_current: String, version_latest: String, build_hash: String, outdated: bool) -> Version {
        Version {
            version_current,
            version_latest,
            build_hash,
            outdated,
        }
    }
}
