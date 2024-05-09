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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "component")]
pub enum FlowChallengeResponseRequest {
    #[serde(rename="ak-source-oauth-apple")]
    AkSourceOauthApple(Box<models::AppleChallengeResponseRequest>),
    #[serde(rename="ak-stage-authenticator-duo")]
    AkStageAuthenticatorDuo(Box<models::AuthenticatorDuoChallengeResponseRequest>),
    #[serde(rename="ak-stage-authenticator-sms")]
    AkStageAuthenticatorSms(Box<models::AuthenticatorSmsChallengeResponseRequest>),
    #[serde(rename="ak-stage-authenticator-static")]
    AkStageAuthenticatorStatic(Box<models::AuthenticatorStaticChallengeResponseRequest>),
    #[serde(rename="ak-stage-authenticator-totp")]
    AkStageAuthenticatorTotp(Box<models::AuthenticatorTotpChallengeResponseRequest>),
    #[serde(rename="ak-stage-authenticator-validate")]
    AkStageAuthenticatorValidate(Box<models::AuthenticatorValidationChallengeResponseRequest>),
    #[serde(rename="ak-stage-authenticator-webauthn")]
    AkStageAuthenticatorWebauthn(Box<models::AuthenticatorWebAuthnChallengeResponseRequest>),
    #[serde(rename="ak-stage-autosubmit")]
    AkStageAutosubmit(Box<models::AutoSubmitChallengeResponseRequest>),
    #[serde(rename="ak-stage-captcha")]
    AkStageCaptcha(Box<models::CaptchaChallengeResponseRequest>),
    #[serde(rename="ak-stage-consent")]
    AkStageConsent(Box<models::ConsentChallengeResponseRequest>),
    #[serde(rename="ak-stage-dummy")]
    AkStageDummy(Box<models::DummyChallengeResponseRequest>),
    #[serde(rename="ak-stage-email")]
    AkStageEmail(Box<models::EmailChallengeResponseRequest>),
    #[serde(rename="ak-stage-identification")]
    AkStageIdentification(Box<models::IdentificationChallengeResponseRequest>),
    #[serde(rename="ak-provider-oauth2-device-code")]
    AkProviderOauth2DeviceCode(Box<models::OAuthDeviceCodeChallengeResponseRequest>),
    #[serde(rename="ak-provider-oauth2-device-code-finish")]
    AkProviderOauth2DeviceCodeFinish(Box<models::OAuthDeviceCodeFinishChallengeResponseRequest>),
    #[serde(rename="ak-stage-password")]
    AkStagePassword(Box<models::PasswordChallengeResponseRequest>),
    #[serde(rename="ak-source-plex")]
    AkSourcePlex(Box<models::PlexAuthenticationChallengeResponseRequest>),
    #[serde(rename="ak-stage-prompt")]
    AkStagePrompt(models::PromptChallengeResponseRequest),
    #[serde(rename="ak-stage-user-login")]
    AkStageUserLogin(Box<models::UserLoginChallengeResponseRequest>),
}

impl Default for FlowChallengeResponseRequest {
    fn default() -> Self {
        Self::AkSourceOauthApple(Default::default())
    }
}


