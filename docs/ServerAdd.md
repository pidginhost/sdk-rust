# ServerAdd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | **String** | ID or slug | 
**package** | **String** | ID or slug | 
**hostname** | Option<**String**> |  | [optional]
**project** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**ssh_pub_key** | Option<**String**> | New SSH key | [optional]
**ssh_pub_key_id** | Option<**String**> | ID or fingerprint | [optional]
**public_ip** | Option<**String**> | ID or slug | [optional]
**new_ipv4** | Option<**bool**> |  | [optional]
**public_ipv6** | Option<**String**> | ID or slug | [optional]
**new_ipv6** | Option<**bool**> |  | [optional]
**fw_rules_set** | Option<**String**> | ID or slug | [optional]
**fw_policy_in** | Option<[**models::FwPolicyOutEnum**](FwPolicyOutEnum.md)> |  | [optional][default to Accept]
**fw_policy_out** | Option<[**models::FwPolicyOutEnum**](FwPolicyOutEnum.md)> |  | [optional][default to Accept]
**private_network** | Option<**String**> | ID or slug | [optional]
**private_address** | Option<**String**> | Leave empty for auto-assign | [optional]
**extra_volume_product** | Option<**String**> | ID or slug | [optional]
**extra_volume_size** | Option<**i32**> |  | [optional][default to 0]
**no_network_acknowledged** | Option<**bool**> |  | [optional]
**enable_ha** | Option<**bool**> |  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


