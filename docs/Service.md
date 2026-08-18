# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**company** | Option<**i32**> |  | [optional]
**billing_cycle** | **i32** |  | [readonly]
**hostname** | **String** |  | [readonly]
**price** | **String** | Euro without TVA | [readonly]
**status** | [**models::ServiceStatusEnum**](ServiceStatusEnum.md) |  | [readonly]
**created** | **String** |  | [readonly]
**modified** | **String** |  | [readonly]
**terminated** | Option<**chrono::NaiveDate**> |  | [readonly]
**next_invoice** | **chrono::NaiveDate** |  | [readonly]
**primary_service** | Option<**i32**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


