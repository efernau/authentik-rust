# Source

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** | Source's display Name. | 
**slug** | **String** | Internal source name, used in URLs. | 
**enabled** | Option<**bool**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled.  * `identifier` - Use the source-specific identifier * `email_link` - Link to a user with identical email address. Can have security implications when a source doesn't validate email addresses. * `email_deny` - Use the user's email address, but deny enrollment when the email address already exists. * `username_link` - Link to a user with identical username. Can have security implications when a username is used with another source. * `username_deny` - Use the user's username, but deny enrollment when the username already exists. | [optional]
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [readonly]
**user_path_template** | Option<**String**> |  | [optional]
**icon** | Option<**String**> | Get the URL to the Icon. If the name is /static or starts with http it is returned as-is | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


