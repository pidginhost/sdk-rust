# Tld

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**tld** | **String** |  | 
**price** | **String** | Euro without TVA | 
**renew_price** | Option<**String**> | Euro without TVA | [optional]
**transfer_price** | Option<**String**> | Euro without TVA | [optional]
**registrar** | **String** |  | 
**api_renewable** | Option<**bool**> |  | [optional]
**idna_support** | Option<**bool**> |  | [optional]
**wg_support** | Option<**bool**> |  | [optional]
**reactivate_max_days** | Option<**i32**> |  | [optional]
**min_years** | Option<**i32**> | Smallest registration term the registry accepts (e.g. 2 for .ai). | [optional]
**year_step** | Option<**i32**> | Allowed step between registration terms (e.g. 2 for .ai → 2, 4, 6 years). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


