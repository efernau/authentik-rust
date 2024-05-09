# Outpost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**r#type** | [**models::OutpostTypeEnum**](OutpostTypeEnum.md) |  | 
**providers** | **Vec<i32>** |  | 
**providers_obj** | [**Vec<models::Provider>**](Provider.md) |  | [readonly]
**service_connection** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Select Service-Connection authentik should use to manage this outpost. Leave empty if authentik should not handle the deployment. | [optional]
**service_connection_obj** | [**models::ServiceConnection**](ServiceConnection.md) |  | [readonly]
**token_identifier** | **String** | Get Token identifier | [readonly]
**config** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


