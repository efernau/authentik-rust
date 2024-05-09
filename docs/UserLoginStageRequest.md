# UserLoginStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**session_duration** | Option<**String**> | Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]
**terminate_other_sessions** | Option<**bool**> | Terminate all other sessions of the user logging in. | [optional]
**remember_me_offset** | Option<**String**> | Offset the session will be extended by when the user picks the remember me option. Default of 0 means that the remember me option will not be shown. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]
**network_binding** | Option<[**models::NetworkBindingEnum**](NetworkBindingEnum.md)> | Bind sessions created by this stage to the configured network  * `no_binding` - No Binding * `bind_asn` - Bind Asn * `bind_asn_network` - Bind Asn Network * `bind_asn_network_ip` - Bind Asn Network Ip | [optional]
**geoip_binding** | Option<[**models::GeoipBindingEnum**](GeoipBindingEnum.md)> | Bind sessions created by this stage to the configured GeoIP location  * `no_binding` - No Binding * `bind_continent` - Bind Continent * `bind_continent_country` - Bind Continent Country * `bind_continent_country_city` - Bind Continent Country City | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


