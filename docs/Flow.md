# Flow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**policybindingmodel_ptr_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**slug** | **String** | Visible in the URL. | 
**title** | **String** | Shown as the Title in Flow pages. | 
**designation** | [**models::FlowDesignationEnum**](FlowDesignationEnum.md) | Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik.  * `authentication` - Authentication * `authorization` - Authorization * `invalidation` - Invalidation * `enrollment` - Enrollment * `unenrollment` - Unrenollment * `recovery` - Recovery * `stage_configuration` - Stage Configuration | 
**background** | **String** | Get the URL to the background image. If the name is /static or starts with http it is returned as-is | [readonly]
**stages** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | [readonly]
**policies** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | [readonly]
**cache_count** | **i32** | Get count of cached flows | [readonly]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**compatibility_mode** | Option<**bool**> | Enable compatibility mode, increases compatibility with password managers on mobile devices. | [optional]
**export_url** | **String** | Get export URL for flow | [readonly]
**layout** | Option<[**models::FlowLayoutEnum**](FlowLayoutEnum.md)> |  | [optional]
**denied_action** | Option<[**models::DeniedActionEnum**](DeniedActionEnum.md)> | Configure what should happen when a flow denies access to a user.  * `message_continue` - Message Continue * `message` - Message * `continue` - Continue | [optional]
**authentication** | Option<[**models::AuthenticationEnum**](AuthenticationEnum.md)> | Required level of authentication and authorization to access a flow.  * `none` - None * `require_authenticated` - Require Authenticated * `require_unauthenticated` - Require Unauthenticated * `require_superuser` - Require Superuser * `require_outpost` - Require Outpost | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


