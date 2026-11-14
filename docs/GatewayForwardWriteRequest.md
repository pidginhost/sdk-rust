# GatewayForwardWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | [**models::GatewayForwardProtocolEnum**](GatewayForwardProtocolEnum.md) |  | 
**public_ports** | [**models::GatewayPortRangeRequest**](GatewayPortRangeRequest.md) |  | 
**target** | [**models::GatewayForwardTargetRequest**](GatewayForwardTargetRequest.md) |  | 
**allowed_sources** | Option<**Vec<String>**> | IPv4 networks; empty means any source. | [optional]
**preserve_client_ip** | Option<**bool**> | Only false is accepted for now. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**comment** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


