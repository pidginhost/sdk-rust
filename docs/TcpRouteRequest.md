# TcpRouteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**namespace** | Option<**String**> |  | [optional]
**port** | **i32** | External port to expose (blocked: 22, 6443, 50000, 50001) | 
**backend_service_name** | **String** | Name of the backend Kubernetes Service | 
**backend_service_port** | **i32** | Port of the backend Service | 
**backend_namespace** | Option<**String**> | Namespace of the backend Service | [optional][default to default]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


