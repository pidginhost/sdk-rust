# PatchedServerDetailRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hostname** | Option<**String**> | Renaming also moves reverse DNS still on the old name. The server takes the new name at its next reboot from the panel or API, which also creates new SSH host keys and resets the login password and SSH keys to the ones saved here. | [optional]
**project** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**ssh_pub_key** | Option<**String**> | Public key to apply for SSH login. Applying a non-empty key regenerates cloud-init and reboots a running server. Clearing removes the key from future cloud-init data, but does not revoke keys already in the guest. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


