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

/// UserOAuthSourceConnection : OAuth Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserOAuthSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: Box<models::Source>,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserOAuthSourceConnection {
    /// OAuth Source Serializer
    pub fn new(pk: i32, user: i32, source: models::Source, identifier: String) -> UserOAuthSourceConnection {
        UserOAuthSourceConnection {
            pk,
            user,
            source: Box::new(source),
            identifier,
        }
    }
}

