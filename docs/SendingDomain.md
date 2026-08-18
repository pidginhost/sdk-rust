# SendingDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**name** | **String** |  | [readonly]
**status** | [**models::SendingDomainStatusEnum**](SendingDomainStatusEnum.md) |  | [readonly]
**dns_source** | [**models::DnsSourceEnum**](DnsSourceEnum.md) |  | [readonly]
**use_inbound** | **bool** |  | [readonly]
**dkim_selector** | **String** |  | [readonly]
**dkim_record** | **String** |  | [readonly]
**spf_record** | **String** |  | [readonly]
**dmarc_record** | **String** |  | [readonly]
**verified_at** | Option<**String**> |  | [readonly]
**last_check_at** | Option<**String**> |  | [readonly]
**last_check_errors** | Option<**serde_json::Value**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


