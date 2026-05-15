# LbFirewallRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**direction** | Option<[**models::LbFirewallRuleDirectionEnum**](LBFirewallRuleDirectionEnum.md)> |  | [optional]
**action** | Option<[**models::LbFirewallRuleActionEnum**](LBFirewallRuleActionEnum.md)> |  | [optional]
**protocol** | Option<**String**> | tcp, udp, icmp, etc. | [optional]
**source** | Option<**String**> | IP address or CIDR | [optional]
**sport** | Option<**String**> | Port or range (e.g., 1024-65535) | [optional]
**destination** | Option<**String**> | IP address or CIDR | [optional]
**dport** | Option<**String**> | Port or range (e.g., 80, 8000-9000) | [optional]
**comment** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**position** | Option<**i32**> | Rule order (lower = higher priority) | [optional]
**created** | **String** |  | [readonly]
**updated** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


