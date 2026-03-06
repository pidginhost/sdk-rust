# PatchedDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**domain** | Option<**String**> |  | [optional][readonly]
**tld** | Option<[**models::Tld**](TLD.md)> |  | [optional][readonly]
**idna** | Option<**bool**> | Domain name is encoded with IDN | [optional][readonly]
**nameservers** | Option<**String**> | List of 2-5 name-servers separated by comma. | [optional]
**expiration_date** | Option<[**String**](String.md)> |  | [optional][readonly]
**registration_date** | Option<[**String**](String.md)> |  | [optional][readonly]
**service** | Option<[**models::Service**](Service.md)> |  | [optional][readonly]
**idna_name** | Option<**String**> |  | [optional][readonly]
**max_renew_years** | Option<**i32**> |  | [optional][readonly]
**service_status** | Option<**String**> | Service status | [optional][readonly]
**contacts** | Option<**serde_json::Value**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


