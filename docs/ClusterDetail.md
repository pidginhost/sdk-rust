# ClusterDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**status** | [**models::ResourceStatusEnum**](ResourceStatusEnum.md) |  | [readonly]
**name** | Option<**String**> |  | [optional]
**generation** | **String** |  | [readonly]
**cluster_type** | Option<**String**> |  | [readonly]
**kube_version** | Option<**String**> |  | [readonly]
**price_per_month** | **String** |  | 
**price_per_hour** | **f64** |  | [readonly]
**features** | Option<[**Vec<models::FeaturesEnum>**](FeaturesEnum.md)> |  | [optional]
**features_ready** | **bool** |  | [readonly]
**kubeconfig_valid_until** | Option<**String**> |  | [readonly]
**ipv4_address** | Option<**String**> |  | [readonly]
**ipv6_address** | Option<**String**> |  | [readonly]
**dual_stack** | **bool** |  | [readonly]
**protected** | Option<**bool**> |  | [optional]
**talos_version** | Option<**String**> |  | [readonly]
**talos_upgrade_available** | **bool** |  | [readonly]
**talos_next_version** | Option<**String**> |  | [readonly]
**storage_quota_gb** | Option<**i32**> |  | [readonly]
**last_pool_used_bytes** | Option<**i64**> |  | [readonly]
**last_storage_sync_at** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


