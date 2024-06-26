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

/// NameIdPolicyEnum : * `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress` - Email * `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent` - Persistent * `urn:oasis:names:tc:SAML:2.0:nameid-format:X509SubjectName` - X509 * `urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName` - Windows * `urn:oasis:names:tc:SAML:2.0:nameid-format:transient` - Transient
/// * `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress` - Email * `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent` - Persistent * `urn:oasis:names:tc:SAML:2.0:nameid-format:X509SubjectName` - X509 * `urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName` - Windows * `urn:oasis:names:tc:SAML:2.0:nameid-format:transient` - Transient
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NameIdPolicyEnum {
    #[serde(rename = "urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress")]
    Variant1Period1ColonNameidFormatColonEmailAddress,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:persistent")]
    Variant2Period0ColonNameidFormatColonPersistent,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:X509SubjectName")]
    Variant2Period0ColonNameidFormatColonX509SubjectName,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName")]
    Variant2Period0ColonNameidFormatColonWindowsDomainQualifiedName,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:transient")]
    Variant2Period0ColonNameidFormatColonTransient,

}

impl ToString for NameIdPolicyEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant1Period1ColonNameidFormatColonEmailAddress => String::from("urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress"),
            Self::Variant2Period0ColonNameidFormatColonPersistent => String::from("urn:oasis:names:tc:SAML:2.0:nameid-format:persistent"),
            Self::Variant2Period0ColonNameidFormatColonX509SubjectName => String::from("urn:oasis:names:tc:SAML:2.0:nameid-format:X509SubjectName"),
            Self::Variant2Period0ColonNameidFormatColonWindowsDomainQualifiedName => String::from("urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName"),
            Self::Variant2Period0ColonNameidFormatColonTransient => String::from("urn:oasis:names:tc:SAML:2.0:nameid-format:transient"),
        }
    }
}

impl Default for NameIdPolicyEnum {
    fn default() -> NameIdPolicyEnum {
        Self::Variant1Period1ColonNameidFormatColonEmailAddress
    }
}

