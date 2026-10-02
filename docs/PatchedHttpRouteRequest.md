# PatchedHttpRouteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**namespace** | Option<**String**> |  | [optional]
**hostnames** | Option<**Vec<String>**> | List of hostnames to route (e.g., [\"example.com\", \"www.example.com\"]) | [optional]
**backend_service_name** | Option<**String**> | Name of the backend Kubernetes Service | [optional]
**backend_service_port** | Option<**i32**> | Port of the backend Service | [optional]
**backend_namespace** | Option<**String**> | Namespace of the backend Service | [optional][default to default]
**path_prefix** | Option<**String**> | Path prefix to match (default: /) | [optional][default to /]
**enable_tls** | Option<**bool**> | Enable TLS termination with automatic certificate issuance | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


