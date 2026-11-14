# RouteStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network** | **i32** |  | 
**gateway** | Option<**i32**> | The gateway that routes for the network, or null. | 
**servers** | [**Vec<models::ServerRoute>**](ServerRoute.md) |  | 
**operations** | [**Vec<models::GatewayOperation>**](GatewayOperation.md) | The network's route applies, newest first. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


