# EndpointRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**provider** | **i32** |  | 
**protocol** | [**models::ProtocolEnum**](ProtocolEnum.md) |  | 
**host** | **String** |  | 
**settings** | Option<[**serde_json::Value**](.md)> |  | [optional]
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**auth_mode** | [**models::AuthModeEnum**](AuthModeEnum.md) |  | 
**maximum_connections** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


