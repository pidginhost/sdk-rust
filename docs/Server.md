# Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**hostname** | Option<**String**> |  | [optional]
**project** | Option<**String**> |  | [optional]
**image** | **String** |  | [readonly]
**package** | **String** |  | [readonly]
**cpus** | **i32** |  | [readonly]
**memory** | **i32** |  | [readonly]
**disk_size** | **i32** |  | [readonly]
**status** | Option<[**models::StatusA57Enum**](StatusA57Enum.md)> |  | [optional]
**destroy_protection** | **bool** | Prevents the server from being destroyed until disabled. | [readonly]
**ha_enabled** | **bool** | Enables Proxmox HA — automatic restart and migration on node failure. | [readonly]
**networks** | **std::collections::HashMap<String, serde_json::Value>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


