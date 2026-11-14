# ActivateFreeDnsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** | Domain name or primary key of the domain to activate. | 
**source** | [**models::SourceEnum**](SourceEnum.md) | 'internal' for domains purchased on PidginHost, 'external' for user-added domains.  * `internal` - Internal * `external` - External | 
**ip** | Option<**String**> | IPv4 address for the zone's default records. Leave it out to start with an empty zone. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


