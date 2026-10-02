# ClusterEncryptionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | [**models::EncryptionModeEnum**](EncryptionModeEnum.md) | Target encryption mode: no encryption, or WireGuard.  * `none` - none * `wireguard` - wireguard | 
**acknowledge_workload_restart** | Option<**bool**> | Confirms the caller accepts that workloads must be restarted after the change. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


