# PublicIpv6

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**slug** | **String** |  | [readonly]
**address** | **String** |  | [readonly]
**gateway** | **String** |  | [readonly]
**prefix** | **i32** |  | [readonly]
**attached** | **bool** |  | [readonly]
**server** | **String** | Hostname of the server this address is attached to. Empty when it is not attached. | [readonly]
**server_id** | Option<**i32**> | ID of the attached server, as used by /api/cloud/servers/{id}/. Null when the address is not attached to a cloud server. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


