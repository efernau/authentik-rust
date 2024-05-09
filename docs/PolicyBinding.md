# PolicyBinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**policy** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user** | Option<**i32**> |  | [optional]
**policy_obj** | [**models::Policy**](Policy.md) |  | [readonly]
**group_obj** | [**models::Group**](Group.md) |  | [readonly]
**user_obj** | [**models::User**](User.md) |  | [readonly]
**target** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**negate** | Option<**bool**> | Negates the outcome of the policy. Messages are unaffected. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**order** | **i32** |  | 
**timeout** | Option<**i32**> | Timeout after which Policy execution is terminated. | [optional]
**failure_result** | Option<**bool**> | Result if the Policy execution fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


