# SamlSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Source's display Name. | 
**slug** | **String** | Internal source name, used in URLs. | 
**enabled** | Option<**bool**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled.  * `identifier` - Use the source-specific identifier * `email_link` - Link to a user with identical email address. Can have security implications when a source doesn't validate email addresses. * `email_deny` - Use the user's email address, but deny enrollment when the email address already exists. * `username_link` - Link to a user with identical username. Can have security implications when a username is used with another source. * `username_deny` - Use the user's username, but deny enrollment when the username already exists. | [optional]
**user_path_template** | Option<**String**> |  | [optional]
**pre_authentication_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used before authentication. | 
**issuer** | Option<**String**> | Also known as Entity ID. Defaults the Metadata URL. | [optional]
**sso_url** | **String** | URL that the initial Login request is sent to. | 
**slo_url** | Option<**String**> | Optional URL if your IDP supports Single-Logout. | [optional]
**allow_idp_initiated** | Option<**bool**> | Allows authentication flows initiated by the IdP. This can be a security risk, as no validation of the request ID is done. | [optional]
**name_id_policy** | Option<[**models::NameIdPolicyEnum**](NameIdPolicyEnum.md)> | NameID Policy sent to the IdP. Can be unset, in which case no Policy is sent.  * `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress` - Email * `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent` - Persistent * `urn:oasis:names:tc:SAML:2.0:nameid-format:X509SubjectName` - X509 * `urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName` - Windows * `urn:oasis:names:tc:SAML:2.0:nameid-format:transient` - Transient | [optional]
**binding_type** | Option<[**models::BindingTypeEnum**](BindingTypeEnum.md)> |  | [optional]
**verification_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default. | [optional]
**signing_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Keypair used to sign outgoing Responses going to the Identity Provider. | [optional]
**digest_algorithm** | Option<[**models::DigestAlgorithmEnum**](DigestAlgorithmEnum.md)> |  | [optional]
**signature_algorithm** | Option<[**models::SignatureAlgorithmEnum**](SignatureAlgorithmEnum.md)> |  | [optional]
**temporary_user_delete_after** | Option<**String**> | Time offset when temporary users should be deleted. This only applies if your IDP uses the NameID Format 'transient', and the user doesn't log out manually. (Format: hours=1;minutes=2;seconds=3). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


