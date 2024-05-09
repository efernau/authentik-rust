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

/// SamlSourceRequest : SAMLSource Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlSourceRequest {
    /// Source's display Name.
    #[serde(rename = "name")]
    pub name: String,
    /// Internal source name, used in URLs.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Flow to use when authenticating existing users.
    #[serde(rename = "authentication_flow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow to use when enrolling new users.
    #[serde(rename = "enrollment_flow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enrollment_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// How the source determines if an existing user should be authenticated or a new user enrolled.  * `identifier` - Use the source-specific identifier * `email_link` - Link to a user with identical email address. Can have security implications when a source doesn't validate email addresses. * `email_deny` - Use the user's email address, but deny enrollment when the email address already exists. * `username_link` - Link to a user with identical username. Can have security implications when a username is used with another source. * `username_deny` - Use the user's username, but deny enrollment when the username already exists.
    #[serde(rename = "user_matching_mode", skip_serializing_if = "Option::is_none")]
    pub user_matching_mode: Option<models::UserMatchingModeEnum>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
    /// Flow used before authentication.
    #[serde(rename = "pre_authentication_flow")]
    pub pre_authentication_flow: uuid::Uuid,
    /// Also known as Entity ID. Defaults the Metadata URL.
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// URL that the initial Login request is sent to.
    #[serde(rename = "sso_url")]
    pub sso_url: String,
    /// Optional URL if your IDP supports Single-Logout.
    #[serde(rename = "slo_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slo_url: Option<Option<String>>,
    /// Allows authentication flows initiated by the IdP. This can be a security risk, as no validation of the request ID is done.
    #[serde(rename = "allow_idp_initiated", skip_serializing_if = "Option::is_none")]
    pub allow_idp_initiated: Option<bool>,
    /// NameID Policy sent to the IdP. Can be unset, in which case no Policy is sent.  * `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress` - Email * `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent` - Persistent * `urn:oasis:names:tc:SAML:2.0:nameid-format:X509SubjectName` - X509 * `urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName` - Windows * `urn:oasis:names:tc:SAML:2.0:nameid-format:transient` - Transient
    #[serde(rename = "name_id_policy", skip_serializing_if = "Option::is_none")]
    pub name_id_policy: Option<models::NameIdPolicyEnum>,
    #[serde(rename = "binding_type", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<models::BindingTypeEnum>,
    /// When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default.
    #[serde(rename = "verification_kp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verification_kp: Option<Option<uuid::Uuid>>,
    /// Keypair used to sign outgoing Responses going to the Identity Provider.
    #[serde(rename = "signing_kp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub signing_kp: Option<Option<uuid::Uuid>>,
    #[serde(rename = "digest_algorithm", skip_serializing_if = "Option::is_none")]
    pub digest_algorithm: Option<models::DigestAlgorithmEnum>,
    #[serde(rename = "signature_algorithm", skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<models::SignatureAlgorithmEnum>,
    /// Time offset when temporary users should be deleted. This only applies if your IDP uses the NameID Format 'transient', and the user doesn't log out manually. (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "temporary_user_delete_after", skip_serializing_if = "Option::is_none")]
    pub temporary_user_delete_after: Option<String>,
}

impl SamlSourceRequest {
    /// SAMLSource Serializer
    pub fn new(name: String, slug: String, pre_authentication_flow: uuid::Uuid, sso_url: String) -> SamlSourceRequest {
        SamlSourceRequest {
            name,
            slug,
            enabled: None,
            authentication_flow: None,
            enrollment_flow: None,
            policy_engine_mode: None,
            user_matching_mode: None,
            user_path_template: None,
            pre_authentication_flow,
            issuer: None,
            sso_url,
            slo_url: None,
            allow_idp_initiated: None,
            name_id_policy: None,
            binding_type: None,
            verification_kp: None,
            signing_kp: None,
            digest_algorithm: None,
            signature_algorithm: None,
            temporary_user_delete_after: None,
        }
    }
}
