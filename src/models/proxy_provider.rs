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

/// ProxyProvider : ProxyProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyProvider {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(rename = "authentication_flow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow")]
    pub authorization_flow: uuid::Uuid,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Get object component so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_application_slug")]
    pub assigned_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_application_name")]
    pub assigned_application_name: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_backchannel_application_slug")]
    pub assigned_backchannel_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_backchannel_application_name")]
    pub assigned_backchannel_application_name: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "internal_host", skip_serializing_if = "Option::is_none")]
    pub internal_host: Option<String>,
    #[serde(rename = "external_host")]
    pub external_host: String,
    /// Validate SSL Certificates of upstream servers
    #[serde(rename = "internal_host_ssl_validation", skip_serializing_if = "Option::is_none")]
    pub internal_host_ssl_validation: Option<bool>,
    #[serde(rename = "certificate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Option<uuid::Uuid>>,
    /// Regular expressions for which authentication is not required. Each new line is interpreted as a new Regular Expression.
    #[serde(rename = "skip_path_regex", skip_serializing_if = "Option::is_none")]
    pub skip_path_regex: Option<String>,
    /// Set a custom HTTP-Basic Authentication header based on values from authentik.
    #[serde(rename = "basic_auth_enabled", skip_serializing_if = "Option::is_none")]
    pub basic_auth_enabled: Option<bool>,
    /// User/Group Attribute used for the password part of the HTTP-Basic Header.
    #[serde(rename = "basic_auth_password_attribute", skip_serializing_if = "Option::is_none")]
    pub basic_auth_password_attribute: Option<String>,
    /// User/Group Attribute used for the user part of the HTTP-Basic Header. If not set, the user's Email address is used.
    #[serde(rename = "basic_auth_user_attribute", skip_serializing_if = "Option::is_none")]
    pub basic_auth_user_attribute: Option<String>,
    /// Enable support for forwardAuth in traefik and nginx auth_request. Exclusive with internal_host.  * `proxy` - Proxy * `forward_single` - Forward Single * `forward_domain` - Forward Domain
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::ProxyMode>,
    /// When enabled, this provider will intercept the authorization header and authenticate requests based on its value.
    #[serde(rename = "intercept_header_auth", skip_serializing_if = "Option::is_none")]
    pub intercept_header_auth: Option<bool>,
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: String,
    #[serde(rename = "cookie_domain", skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,
    #[serde(rename = "jwks_sources", skip_serializing_if = "Option::is_none")]
    pub jwks_sources: Option<Vec<uuid::Uuid>>,
    /// Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "access_token_validity", skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<String>,
    /// Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "refresh_token_validity", skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<String>,
    #[serde(rename = "outpost_set")]
    pub outpost_set: Vec<String>,
}

impl ProxyProvider {
    /// ProxyProvider Serializer
    pub fn new(pk: i32, name: String, authorization_flow: uuid::Uuid, component: String, assigned_application_slug: String, assigned_application_name: String, assigned_backchannel_application_slug: String, assigned_backchannel_application_name: String, verbose_name: String, verbose_name_plural: String, meta_model_name: String, client_id: String, external_host: String, redirect_uris: String, outpost_set: Vec<String>) -> ProxyProvider {
        ProxyProvider {
            pk,
            name,
            authentication_flow: None,
            authorization_flow,
            property_mappings: None,
            component,
            assigned_application_slug,
            assigned_application_name,
            assigned_backchannel_application_slug,
            assigned_backchannel_application_name,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            client_id,
            internal_host: None,
            external_host,
            internal_host_ssl_validation: None,
            certificate: None,
            skip_path_regex: None,
            basic_auth_enabled: None,
            basic_auth_password_attribute: None,
            basic_auth_user_attribute: None,
            mode: None,
            intercept_header_auth: None,
            redirect_uris,
            cookie_domain: None,
            jwks_sources: None,
            access_token_validity: None,
            refresh_token_validity: None,
            outpost_set,
        }
    }
}
