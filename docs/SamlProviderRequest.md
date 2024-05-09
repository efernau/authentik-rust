# SamlProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used when authorizing this provider. | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**acs_url** | **String** |  | 
**audience** | Option<**String**> | Value of the audience restriction field of the assertion. When left empty, no audience restriction will be added. | [optional]
**issuer** | Option<**String**> | Also known as EntityID | [optional]
**assertion_valid_not_before** | Option<**String**> | Assertion valid not before current time + this value (Format: hours=-1;minutes=-2;seconds=-3). | [optional]
**assertion_valid_not_on_or_after** | Option<**String**> | Assertion not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**session_valid_not_on_or_after** | Option<**String**> | Session not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**name_id_mapping** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Configure how the NameID value will be created. When left empty, the NameIDPolicy of the incoming request will be considered | [optional]
**digest_algorithm** | Option<[**models::DigestAlgorithmEnum**](DigestAlgorithmEnum.md)> |  | [optional]
**signature_algorithm** | Option<[**models::SignatureAlgorithmEnum**](SignatureAlgorithmEnum.md)> |  | [optional]
**signing_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Keypair used to sign outgoing Responses going to the Service Provider. | [optional]
**verification_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default. | [optional]
**sp_binding** | Option<[**models::SpBindingEnum**](SpBindingEnum.md)> | This determines how authentik sends the response back to the Service Provider.  * `redirect` - Redirect * `post` - Post | [optional]
**default_relay_state** | Option<**String**> | Default relay_state value for IDP-initiated logins | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


