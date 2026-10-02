# DedicatedServer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**hostname** | **String** |  | [readonly]
**status** | [**models::ServiceStatusEnum**](ServiceStatusEnum.md) |  | [readonly]
**price** | **String** | Euro without TVA | [readonly]
**next_invoice** | **chrono::NaiveDate** |  | [readonly]
**created** | **String** |  | [readonly]
**billing_cycle** | **String** |  | [readonly]
**server_status** | Option<[**models::DedicatedServerStatus**](DedicatedServerStatus.md)> |  | [readonly]
**ips** | [**Vec<models::DedicatedServerIp>**](DedicatedServerIP.md) |  | [readonly]
**os_name** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


