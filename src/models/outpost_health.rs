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

/// OutpostHealth : Outpost health status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutpostHealth {
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "version_should")]
    pub version_should: String,
    #[serde(rename = "version_outdated")]
    pub version_outdated: bool,
    #[serde(rename = "build_hash")]
    pub build_hash: String,
    #[serde(rename = "build_hash_should")]
    pub build_hash_should: String,
    #[serde(rename = "hostname")]
    pub hostname: String,
}

impl OutpostHealth {
    /// Outpost health status
    pub fn new(uid: String, last_seen: String, version: String, version_should: String, version_outdated: bool, build_hash: String, build_hash_should: String, hostname: String) -> OutpostHealth {
        OutpostHealth {
            uid,
            last_seen,
            version,
            version_should,
            version_outdated,
            build_hash,
            build_hash_should,
            hostname,
        }
    }
}
