# Domain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**domain** | **String** |  | [readonly]
**tld** | [**models::Tld**](TLD.md) |  | [readonly]
**idna** | **bool** | Domain name is encoded with IDN | [readonly]
**nameservers** | Option<**String**> | List of 2-5 name-servers separated by comma. | [optional]
**expiration_date** | [**String**](String.md) |  | [readonly]
**registration_date** | Option<[**String**](String.md)> |  | [readonly]
**service** | [**models::Service**](Service.md) |  | [readonly]
**idna_name** | **String** |  | [readonly]
**max_renew_years** | **i32** |  | [readonly]
**service_status** | **String** | Service status | [readonly]
**contacts** | Option<**serde_json::Value**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


