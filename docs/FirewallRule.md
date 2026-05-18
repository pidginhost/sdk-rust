# FirewallRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**direction** | [**models::FirewallRuleDirectionEnum**](FirewallRuleDirectionEnum.md) |  | 
**action** | [**models::FwPolicyOutEnum**](FwPolicyOutEnum.md) |  | 
**protocol** | Option<**String**> |  | [optional]
**source** | Option<**String**> | single IP, range (20.34.101.207-201.3.9.99) or comma separated list | [optional]
**sport** | Option<**String**> | numbers (0-65535), range (\"\\d+:\\d+\", like \"80:85\"), comma separated list | [optional]
**destination** | Option<**String**> | single IP, range (20.34.101.207-201.3.9.99) or comma separated list | [optional]
**dport** | Option<**String**> | numbers (0-65535), range (\"\\d+:\\d+\", like \"80:85\"), comma separated list | [optional]
**enabled** | Option<**bool**> |  | [optional]
**position** | Option<**i32**> |  | [optional]
**has_error** | **bool** |  | [readonly]
**error_message** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


