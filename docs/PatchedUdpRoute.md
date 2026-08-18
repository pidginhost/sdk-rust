# PatchedUdpRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**namespace** | Option<**String**> |  | [optional]
**port** | Option<**i32**> | External port to expose | [optional]
**backend_service_name** | Option<**String**> | Name of the backend Kubernetes Service | [optional]
**backend_service_port** | Option<**i32**> | Port of the backend Service | [optional]
**backend_namespace** | Option<**String**> | Namespace of the backend Service | [optional][default to default]
**status_ready** | Option<**bool**> |  | [optional][readonly]
**status_message** | Option<**String**> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


