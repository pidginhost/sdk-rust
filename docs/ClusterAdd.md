# ClusterAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_type** | [**models::ClusterTypeEnum**](ClusterTypeEnum.md) |  | 
**name** | Option<**String**> |  | [optional]
**resource_pool_package** | **String** | ID or slug | 
**resource_pool_size** | Option<**i32**> |  | [optional]
**kube_version** | Option<[**models::KubeVersionEnum**](KubeVersionEnum.md)> |  | [optional][default to Variant1354]
**features** | Option<[**Vec<models::FeaturesEnum>**](FeaturesEnum.md)> |  | [optional]
**enable_gateway_api** | Option<**bool**> |  | [optional]
**dual_stack** | Option<**bool**> | Enable IPv6 dual-stack for pods, services, and the cluster private network. Available only when the platform has K8S_DUAL_STACK_ENABLED. Cannot be changed after provisioning. | [optional][default to false]
**generation** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


