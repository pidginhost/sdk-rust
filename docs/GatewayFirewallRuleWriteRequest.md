# GatewayFirewallRuleWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | [**models::FwPolicyOutEnum**](FwPolicyOutEnum.md) |  | 
**source** | [**models::GatewayFirewallSourceRequest**](GatewayFirewallSourceRequest.md) |  | 
**destination** | [**models::GatewayFirewallDestinationRequest**](GatewayFirewallDestinationRequest.md) |  | 
**protocol** | [**models::GatewayFirewallProtocolEnum**](GatewayFirewallProtocolEnum.md) |  | 
**ports** | Option<[**Vec<models::GatewayPortRangeRequest>**](GatewayPortRangeRequest.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**comment** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


