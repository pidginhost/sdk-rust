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
**floating_ips** | Option<[**Vec<models::FloatingIpSummary>**](FloatingIPSummary.md)> |  | [optional][readonly]
**password** | Option<**String**> |  | [optional]
**ssh_pub_key** | Option<**String**> | Public key to apply for SSH login. Applying a non-empty key regenerates cloud-init and reboots a running server. Clearing removes the key from future cloud-init data, but does not revoke keys already in the guest. | [optional]
**status** | Option<[**models::ResourceStatusEnum**](ResourceStatusEnum.md)> |  | [optional][readonly]
**username** | Option<**String**> |  | [optional][readonly]
**destroy_protection** | Option<**bool**> | Prevents the server from being destroyed until disabled. | [optional][readonly]
**ha_enabled** | Option<**bool**> | Enables Proxmox HA — automatic restart and migration on node failure. | [optional][readonly]
**custom_os** | Option<**bool**> | Customer installed their own OS from an ISO; cloud-init features no longer apply | [optional][readonly]
**rescue_mode** | Option<**bool**> |  | [optional][readonly]
**boot_iso** | Option<**String**> |  | [optional][readonly]
**rescue_supported** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


