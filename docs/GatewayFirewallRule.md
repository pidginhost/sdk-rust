# GatewayFirewallRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**position** | **i32** |  | [readonly]
**action** | [**models::FwPolicyOutEnum**](FwPolicyOutEnum.md) |  | [readonly]
**source** | [**models::GatewayFirewallSource**](GatewayFirewallSource.md) |  | [readonly]
**destination** | [**models::GatewayFirewallDestination**](GatewayFirewallDestination.md) |  | [readonly]
**protocol** | [**models::GatewayFirewallProtocolEnum**](GatewayFirewallProtocolEnum.md) |  | [readonly]
**ports** | [**Vec<models::GatewayPortRange>**](GatewayPortRange.md) |  | [readonly]
**enabled** | **bool** |  | [readonly]
**comment** | **String** |  | [readonly]
**changed_revision** | **i32** |  | [readonly]
**pending** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


