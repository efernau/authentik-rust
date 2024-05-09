# IdentificationStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**user_fields** | Option<[**Vec<models::UserFieldsEnum>**](UserFieldsEnum.md)> | Fields of the user object to match against. (Hold shift to select multiple options) | [optional]
**password_stage** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When set, shows a password field, instead of showing the password field as seaprate step. | [optional]
**case_insensitive_matching** | Option<**bool**> | When enabled, user fields are matched regardless of their casing. | [optional]
**show_matched_user** | Option<**bool**> | When a valid username/email has been entered, and this option is enabled, the user's username and avatar will be shown. Otherwise, the text that the user entered will be shown | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional enrollment flow, which is linked at the bottom of the page. | [optional]
**recovery_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional recovery flow, which is linked at the bottom of the page. | [optional]
**passwordless_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional passwordless flow, which is linked at the bottom of the page. | [optional]
**sources** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Specify which sources should be shown. | [optional]
**show_source_labels** | Option<**bool**> |  | [optional]
**pretend_user_exists** | Option<**bool**> | When enabled, the stage will succeed and continue even when incorrect user info is entered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


