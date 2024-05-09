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

/// BackendsEnum : * `authentik.core.auth.InbuiltBackend` - User database + standard password * `authentik.core.auth.TokenBackend` - User database + app passwords * `authentik.sources.ldap.auth.LDAPBackend` - User database + LDAP password
/// * `authentik.core.auth.InbuiltBackend` - User database + standard password * `authentik.core.auth.TokenBackend` - User database + app passwords * `authentik.sources.ldap.auth.LDAPBackend` - User database + LDAP password
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackendsEnum {
    #[serde(rename = "authentik.core.auth.InbuiltBackend")]
    CorePeriodAuthPeriodInbuiltBackend,
    #[serde(rename = "authentik.core.auth.TokenBackend")]
    CorePeriodAuthPeriodTokenBackend,
    #[serde(rename = "authentik.sources.ldap.auth.LDAPBackend")]
    SourcesPeriodLdapPeriodAuthPeriodLdapBackend,

}

impl ToString for BackendsEnum {
    fn to_string(&self) -> String {
        match self {
            Self::CorePeriodAuthPeriodInbuiltBackend => String::from("authentik.core.auth.InbuiltBackend"),
            Self::CorePeriodAuthPeriodTokenBackend => String::from("authentik.core.auth.TokenBackend"),
            Self::SourcesPeriodLdapPeriodAuthPeriodLdapBackend => String::from("authentik.sources.ldap.auth.LDAPBackend"),
        }
    }
}

impl Default for BackendsEnum {
    fn default() -> BackendsEnum {
        Self::CorePeriodAuthPeriodInbuiltBackend
    }
}
