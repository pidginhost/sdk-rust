# Gateway

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**name** | **String** |  | [readonly]
**state** | [**models::GatewayStateEnum**](GatewayStateEnum.md) |  | [readonly]
**state_reason** | **String** |  | [readonly]
**status** | [**models::ResourceStatusEnum**](ResourceStatusEnum.md) |  | [readonly]
**desired_revision** | **i32** |  | [readonly]
**applied_revision** | Option<**i32**> |  | [readonly]
**ipv4** | Option<**String**> |  | [readonly]
**ipv6** | Option<**String**> |  | [readonly]
**maintenance_weekday** | **i32** |  | [readonly]
**maintenance_hour** | **i32** |  | [readonly]
**ha_enabled** | **bool** |  | [readonly]
**current_operation** | Option<**uuid::Uuid**> |  | [readonly]
**created** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


