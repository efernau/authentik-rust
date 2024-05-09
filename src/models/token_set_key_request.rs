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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenSetKeyRequest {
    #[serde(rename = "key")]
    pub key: String,
}

impl TokenSetKeyRequest {
    pub fn new(key: String) -> TokenSetKeyRequest {
        TokenSetKeyRequest {
            key,
        }
    }
}

