# AuthenticatorWebAuthnChallenge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::ChallengeChoices**](ChallengeChoices.md) |  | 
**flow_info** | Option<[**models::ContextualFlowInfo**](ContextualFlowInfo.md)> |  | [optional]
**component** | Option<**String**> |  | [optional][default to ak-stage-authenticator-webauthn]
**response_errors** | Option<[**std::collections::HashMap<String, Vec<models::ErrorDetail>>**](Vec.md)> |  | [optional]
**pending_user** | **String** |  | 
**pending_user_avatar** | **String** |  | 
**registration** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


