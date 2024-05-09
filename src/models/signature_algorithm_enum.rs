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

/// SignatureAlgorithmEnum : * `http://www.w3.org/2000/09/xmldsig#rsa-sha1` - RSA-SHA1 * `http://www.w3.org/2001/04/xmldsig-more#rsa-sha256` - RSA-SHA256 * `http://www.w3.org/2001/04/xmldsig-more#rsa-sha384` - RSA-SHA384 * `http://www.w3.org/2001/04/xmldsig-more#rsa-sha512` - RSA-SHA512 * `http://www.w3.org/2000/09/xmldsig#dsa-sha1` - DSA-SHA1
/// * `http://www.w3.org/2000/09/xmldsig#rsa-sha1` - RSA-SHA1 * `http://www.w3.org/2001/04/xmldsig-more#rsa-sha256` - RSA-SHA256 * `http://www.w3.org/2001/04/xmldsig-more#rsa-sha384` - RSA-SHA384 * `http://www.w3.org/2001/04/xmldsig-more#rsa-sha512` - RSA-SHA512 * `http://www.w3.org/2000/09/xmldsig#dsa-sha1` - DSA-SHA1
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignatureAlgorithmEnum {
    #[serde(rename = "http://www.w3.org/2000/09/xmldsig#rsa-sha1")]
    Variant2000Slash09SlashXmldsigHashRsaSha1,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")]
    Variant2001Slash04SlashXmldsigMoreHashRsaSha256,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha384")]
    Variant2001Slash04SlashXmldsigMoreHashRsaSha384,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512")]
    Variant2001Slash04SlashXmldsigMoreHashRsaSha512,
    #[serde(rename = "http://www.w3.org/2000/09/xmldsig#dsa-sha1")]
    Variant2000Slash09SlashXmldsigHashDsaSha1,

}

impl ToString for SignatureAlgorithmEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant2000Slash09SlashXmldsigHashRsaSha1 => String::from("http://www.w3.org/2000/09/xmldsig#rsa-sha1"),
            Self::Variant2001Slash04SlashXmldsigMoreHashRsaSha256 => String::from("http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"),
            Self::Variant2001Slash04SlashXmldsigMoreHashRsaSha384 => String::from("http://www.w3.org/2001/04/xmldsig-more#rsa-sha384"),
            Self::Variant2001Slash04SlashXmldsigMoreHashRsaSha512 => String::from("http://www.w3.org/2001/04/xmldsig-more#rsa-sha512"),
            Self::Variant2000Slash09SlashXmldsigHashDsaSha1 => String::from("http://www.w3.org/2000/09/xmldsig#dsa-sha1"),
        }
    }
}

impl Default for SignatureAlgorithmEnum {
    fn default() -> SignatureAlgorithmEnum {
        Self::Variant2000Slash09SlashXmldsigHashRsaSha1
    }
}
