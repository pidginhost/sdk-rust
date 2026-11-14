# GatewayLbListenerWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | Option<[**models::GatewayForwardProtocolEnum**](GatewayForwardProtocolEnum.md)> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**mode** | Option<[**models::GatewayLbListenerModeEnum**](GatewayLBListenerModeEnum.md)> | tcp, udp or http; https is not available yet.  * `tcp` - tcp * `udp` - udp * `http` - http * `https` - https | [optional]
**pool** | Option<**i32**> | A backend pool's id; tcp and udp only. | [optional]
**allowed_sources** | Option<**Vec<String>**> | IPv4 and IPv6 networks; empty means any source. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**comment** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


