# GatewayLbListener

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**protocol** | [**models::GatewayForwardProtocolEnum**](GatewayForwardProtocolEnum.md) |  | [readonly]
**port** | **i32** |  | 
**mode** | [**models::GatewayLbListenerModeEnum**](GatewayLBListenerModeEnum.md) |  | 
**pool** | Option<**i32**> | The backend pool of a tcp or udp listener; null for http. | 
**allowed_sources** | **Vec<String>** | Empty means any source. | 
**enabled** | **bool** |  | [readonly]
**comment** | **String** |  | [readonly]
**changed_revision** | **i32** |  | [readonly]
**pending** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


