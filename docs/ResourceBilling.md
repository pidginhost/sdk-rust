# ResourceBilling

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** |  | 
**currency** | **String** |  | 
**tax_included** | **bool** |  | 
**cycle_months** | **i32** |  | 
**cycle_label** | **String** |  | 
**next_invoice** | Option<**chrono::NaiveDate**> |  | 
**profile** | Option<[**models::ResourceBillingProfile**](ResourceBillingProfile.md)> |  | 
**metered** | **bool** |  | 
**included** | **bool** |  | 
**hourly_amount** | Option<**String**> |  | [optional]
**monthly_total** | Option<**String**> |  | [optional]
**components** | Option<**std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


