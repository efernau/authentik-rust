# FlowSetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**slug** | **String** | Visible in the URL. | 
**title** | **String** | Shown as the Title in Flow pages. | 
**designation** | [**models::FlowDesignationEnum**](FlowDesignationEnum.md) | Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik.  * `authentication` - Authentication * `authorization` - Authorization * `invalidation` - Invalidation * `enrollment` - Enrollment * `unenrollment` - Unrenollment * `recovery` - Recovery * `stage_configuration` - Stage Configuration | 
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**compatibility_mode** | Option<**bool**> | Enable compatibility mode, increases compatibility with password managers on mobile devices. | [optional]
**layout** | Option<[**models::FlowLayoutEnum**](FlowLayoutEnum.md)> |  | [optional]
**denied_action** | Option<[**models::DeniedActionEnum**](DeniedActionEnum.md)> | Configure what should happen when a flow denies access to a user.  * `message_continue` - Message Continue * `message` - Message * `continue` - Continue | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


