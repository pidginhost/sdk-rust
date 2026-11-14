# GatewayHealthCheckWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**models::GatewayHealthCheckTypeEnum**](GatewayHealthCheckTypeEnum.md)> |  | [optional]
**path** | Option<**String**> | http only. | [optional]
**status_min** | Option<**i32**> | http only. | [optional]
**status_max** | Option<**i32**> | http only. | [optional]
**interval_s** | Option<**i32**> |  | [optional]
**timeout_s** | Option<**i32**> | Below interval_s. | [optional]
**healthy_threshold** | Option<**i32**> |  | [optional]
**unhealthy_threshold** | Option<**i32**> |  | [optional]
**port** | Option<**i32**> | Null checks each server's own port. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


