# ServerDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**hostname** | **String** |  | [readonly]
**project** | Option<**String**> |  | [optional]
**image** | **String** |  | [readonly]
**package** | **String** |  | [readonly]
**cpus** | **i32** |  | [readonly]
**memory** | **i32** |  | [readonly]
**disk_size** | **i32** |  | [readonly]
**generation** | **String** |  | [readonly]
**machine** | **std::collections::HashMap<String, serde_json::Value>** |  | [readonly]
**volumes** | [**Vec<models::Volume>**](Volume.md) |  | [readonly]
**networks** | **std::collections::HashMap<String, serde_json::Value>** |  | [readonly]
**floating_ips** | [**Vec<models::FloatingIpSummary>**](FloatingIPSummary.md) |  | [readonly]
**password** | Option<**String**> |  | [optional]
**ssh_pub_key** | Option<**String**> | Public key to apply for SSH login. Applying a non-empty key regenerates cloud-init and reboots a running server. Clearing removes the key from future cloud-init data, but does not revoke keys already in the guest. | [optional]
**status** | [**models::ResourceStatusEnum**](ResourceStatusEnum.md) |  | [readonly]
**username** | **String** |  | [readonly]
**destroy_protection** | **bool** | Prevents the server from being destroyed until disabled. | [readonly]
**ha_enabled** | **bool** | Enables Proxmox HA — automatic restart and migration on node failure. | [readonly]
**custom_os** | **bool** | Customer installed their own OS from an ISO; cloud-init features no longer apply | [readonly]
**rescue_mode** | **bool** |  | [readonly]
**boot_iso** | Option<**String**> |  | [readonly]
**rescue_supported** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


