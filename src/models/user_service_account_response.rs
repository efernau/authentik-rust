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
pub struct UserServiceAccountResponse {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "user_uid")]
    pub user_uid: String,
    #[serde(rename = "user_pk")]
    pub user_pk: i32,
    #[serde(rename = "group_pk", skip_serializing_if = "Option::is_none")]
    pub group_pk: Option<String>,
}

impl UserServiceAccountResponse {
    pub fn new(username: String, token: String, user_uid: String, user_pk: i32) -> UserServiceAccountResponse {
        UserServiceAccountResponse {
            username,
            token,
            user_uid,
            user_pk,
            group_pk: None,
        }
    }
}

