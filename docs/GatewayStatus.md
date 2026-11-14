# GatewayStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | [**models::GatewayStateEnum**](GatewayStateEnum.md) |  | 
**state_reason** | **String** |  | 
**state_changed_at** | Option<**String**> |  | 
**desired_revision** | **i32** |  | 
**applied_revision** | Option<**i32**> |  | 
**last_status_at** | Option<**String**> |  | 
**agent_version** | Option<**String**> |  | 
**envoy_version** | Option<**String**> |  | 
**image_version** | Option<**String**> |  | 
**attempted_revision** | Option<**i32**> |  | 
**local_ready** | Option<**bool**> |  | 
**clock_synchronized** | Option<**bool**> |  | 
**apply_error** | Option<[**models::GatewayApplyError**](GatewayApplyError.md)> |  | 
**interfaces** | Option<[**Vec<models::GatewayInterface>**](GatewayInterface.md)> |  | 
**conntrack** | Option<[**models::GatewayConntrack**](GatewayConntrack.md)> |  | 
**egress_probe** | Option<[**models::GatewayEgressProbe**](GatewayEgressProbe.md)> |  | 
**security_updates** | Option<**i32**> |  | 
**reboot_required** | Option<**bool**> |  | 
**uptime_s** | Option<**i32**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


