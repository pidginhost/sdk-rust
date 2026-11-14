# GatewayForward

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**protocol** | [**models::GatewayForwardProtocolEnum**](GatewayForwardProtocolEnum.md) |  | [readonly]
**public_ports** | [**models::GatewayPortRange**](GatewayPortRange.md) |  | [readonly]
**allowed_sources** | Option<**serde_json::Value**> |  | [readonly]
**target** | [**models::GatewayForwardTarget**](GatewayForwardTarget.md) |  | [readonly]
**preserve_client_ip** | **bool** | Always false: the target sees the gateway's address. | [readonly]
**enabled** | **bool** |  | [readonly]
**comment** | **String** |  | [readonly]
**changed_revision** | **i32** |  | [readonly]
**pending** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


