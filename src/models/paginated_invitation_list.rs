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
pub struct PaginatedInvitationList {
    #[serde(rename = "pagination")]
    pub pagination: Box<models::Pagination>,
    #[serde(rename = "results")]
    pub results: Vec<models::Invitation>,
}

impl PaginatedInvitationList {
    pub fn new(pagination: models::Pagination, results: Vec<models::Invitation>) -> PaginatedInvitationList {
        PaginatedInvitationList {
            pagination: Box::new(pagination),
            results,
        }
    }
}

