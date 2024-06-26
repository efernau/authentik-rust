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

/// ProviderTypeEnum : * `apple` - Apple * `openidconnect` - OpenID Connect * `azuread` - Azure AD * `discord` - Discord * `facebook` - Facebook * `github` - GitHub * `gitlab` - GitLab * `google` - Google * `mailcow` - Mailcow * `okta` - Okta * `patreon` - Patreon * `reddit` - Reddit * `twitch` - Twitch * `twitter` - Twitter
/// * `apple` - Apple * `openidconnect` - OpenID Connect * `azuread` - Azure AD * `discord` - Discord * `facebook` - Facebook * `github` - GitHub * `gitlab` - GitLab * `google` - Google * `mailcow` - Mailcow * `okta` - Okta * `patreon` - Patreon * `reddit` - Reddit * `twitch` - Twitch * `twitter` - Twitter
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTypeEnum {
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "openidconnect")]
    Openidconnect,
    #[serde(rename = "azuread")]
    Azuread,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "mailcow")]
    Mailcow,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "patreon")]
    Patreon,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "twitter")]
    Twitter,

}

impl ToString for ProviderTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Apple => String::from("apple"),
            Self::Openidconnect => String::from("openidconnect"),
            Self::Azuread => String::from("azuread"),
            Self::Discord => String::from("discord"),
            Self::Facebook => String::from("facebook"),
            Self::Github => String::from("github"),
            Self::Gitlab => String::from("gitlab"),
            Self::Google => String::from("google"),
            Self::Mailcow => String::from("mailcow"),
            Self::Okta => String::from("okta"),
            Self::Patreon => String::from("patreon"),
            Self::Reddit => String::from("reddit"),
            Self::Twitch => String::from("twitch"),
            Self::Twitter => String::from("twitter"),
        }
    }
}

impl Default for ProviderTypeEnum {
    fn default() -> ProviderTypeEnum {
        Self::Apple
    }
}

