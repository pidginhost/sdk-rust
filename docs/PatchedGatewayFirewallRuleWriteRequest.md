# PatchedGatewayFirewallRuleWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<[**models::FwPolicyOutEnum**](FwPolicyOutEnum.md)> |  | [optional]
**source** | Option<[**models::GatewayFirewallSourceRequest**](GatewayFirewallSourceRequest.md)> |  | [optional]
**destination** | Option<[**models::GatewayFirewallDestinationRequest**](GatewayFirewallDestinationRequest.md)> |  | [optional]
**protocol** | Option<[**models::GatewayFirewallProtocolEnum**](GatewayFirewallProtocolEnum.md)> |  | [optional]
**ports** | Option<[**Vec<models::GatewayPortRangeRequest>**](GatewayPortRangeRequest.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**comment** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


