# FlowStageBinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**policybindingmodel_ptr_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**target** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**stage** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**stage_obj** | [**models::Stage**](Stage.md) |  | [readonly]
**evaluate_on_plan** | Option<**bool**> | Evaluate policies during the Flow planning process. | [optional]
**re_evaluate_policies** | Option<**bool**> | Evaluate policies when the Stage is present to the user. | [optional]
**order** | **i32** |  | 
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**invalid_response_action** | Option<[**models::InvalidResponseActionEnum**](InvalidResponseActionEnum.md)> | Configure how the flow executor should handle an invalid response to a challenge. RETRY returns the error message and a similar challenge to the executor. RESTART restarts the flow from the beginning, and RESTART_WITH_CONTEXT restarts the flow while keeping the current context.  * `retry` - Retry * `restart` - Restart * `restart_with_context` - Restart With Context | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


