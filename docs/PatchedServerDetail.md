# PatchedServerDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**hostname** | Option<**String**> |  | [optional][readonly]
**project** | Option<**String**> |  | [optional]
**image** | Option<**String**> |  | [optional][readonly]
**package** | Option<**String**> |  | [optional][readonly]
**cpus** | Option<**i32**> |  | [optional][readonly]
**memory** | Option<**i32**> |  | [optional][readonly]
**disk_size** | Option<**i32**> |  | [optional][readonly]
**generation** | Option<**String**> |  | [optional][readonly]
**machine** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional][readonly]
**volumes** | Option<[**Vec<models::Volume>**](Volume.md)> |  | [optional][readonly]
**networks** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional][readonly]
**password** | Option<**String**> |  | [optional]
**status** | Option<[**models::StatusA57Enum**](StatusA57Enum.md)> |  | [optional][readonly]
**username** | Option<**String**> |  | [optional][readonly]
**destroy_protection** | Option<**bool**> | Prevents the server from being destroyed until disabled. | [optional][readonly]
**ha_enabled** | Option<**bool**> | Enables Proxmox HA — automatic restart and migration on node failure. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


