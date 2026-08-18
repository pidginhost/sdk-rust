# ClusterDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**status** | [**models::ResourceStatusEnum**](ResourceStatusEnum.md) |  | [readonly]
**name** | Option<**String**> |  | [optional]
**generation** | **String** |  | [readonly]
**cluster_type** | **String** |  | [readonly]
**kube_version** | **String** |  | [readonly]
**price_per_month** | **String** |  | 
**price_per_hour** | **f64** |  | [readonly]
**features** | Option<[**Vec<models::FeaturesEnum>**](FeaturesEnum.md)> |  | [optional]
**features_ready** | **bool** |  | [readonly]
**kubeconfig_valid_until** | **String** |  | [readonly]
**ipv4_address** | **String** |  | [readonly]
**ipv6_address** | **String** |  | [readonly]
**dual_stack** | **bool** |  | [readonly]
**protected** | Option<**bool**> |  | [optional]
**talos_version** | **String** |  | [readonly]
**talos_upgrade_available** | **bool** |  | [readonly]
**talos_next_version** | **String** |  | [readonly]
**storage_quota_gb** | **i32** |  | [readonly]
**last_pool_used_bytes** | **i64** |  | [readonly]
**last_storage_sync_at** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


