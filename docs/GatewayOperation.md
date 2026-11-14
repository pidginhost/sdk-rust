# GatewayOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**kind** | [**models::GatewayOperationKindEnum**](GatewayOperationKindEnum.md) |  | [readonly]
**status** | [**models::GatewayOperationStatusEnum**](GatewayOperationStatusEnum.md) |  | [readonly]
**network** | Option<**i32**> |  | [readonly]
**revision** | Option<**i32**> |  | [readonly]
**started_at** | **String** |  | [readonly]
**deadline_at** | Option<**String**> |  | [readonly]
**completed_at** | Option<**String**> |  | [readonly]
**servers** | **Vec<i32>** |  | [readonly]
**outcomes** | [**std::collections::HashMap<String, models::GatewayRouteOutcome>**](GatewayRouteOutcome.md) |  | [readonly]
**reason** | **String** | Why the operation failed; empty unless it did. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


