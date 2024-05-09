# PatchedOAuth2ProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used when authorizing this provider. | [optional]
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**client_type** | Option<[**models::ClientTypeEnum**](ClientTypeEnum.md)> | Confidential clients are capable of maintaining the confidentiality of their credentials. Public clients are incapable  * `confidential` - Confidential * `public` - Public | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**access_code_validity** | Option<**String**> | Access codes not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**access_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**refresh_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**include_claims_in_id_token** | Option<**bool**> | Include User claims from scopes in the id_token, for applications that don't access the userinfo endpoint. | [optional]
**signing_key** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Key used to sign the tokens. Only required when JWT Algorithm is set to RS256. | [optional]
**redirect_uris** | Option<**String**> | Enter each URI on a new line. | [optional]
**sub_mode** | Option<[**models::SubModeEnum**](SubModeEnum.md)> | Configure what data should be used as unique User Identifier. For most cases, the default should be fine.  * `hashed_user_id` - Based on the Hashed User ID * `user_id` - Based on user ID * `user_uuid` - Based on user UUID * `user_username` - Based on the username * `user_email` - Based on the User's Email. This is recommended over the UPN method. * `user_upn` - Based on the User's UPN, only works if user has a 'upn' attribute set. Use this method only if you have different UPN and Mail domains. | [optional]
**issuer_mode** | Option<[**models::IssuerModeEnum**](IssuerModeEnum.md)> | Configure how the issuer field of the ID Token should be filled.  * `global` - Same identifier is used for all providers * `per_provider` - Each provider has a different issuer, based on the application slug. | [optional]
**jwks_sources** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


