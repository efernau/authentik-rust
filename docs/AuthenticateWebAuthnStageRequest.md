# AuthenticateWebAuthnStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**configure_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**user_verification** | Option<[**models::UserVerificationEnum**](UserVerificationEnum.md)> |  | [optional]
**authenticator_attachment** | Option<[**models::AuthenticatorAttachmentEnum**](AuthenticatorAttachmentEnum.md)> |  | [optional]
**resident_key_requirement** | Option<[**models::ResidentKeyRequirementEnum**](ResidentKeyRequirementEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


