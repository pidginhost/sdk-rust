# ServerTrafficResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**year** | **i32** |  | 
**month** | **i32** |  | 
**as_of** | Option<**chrono::NaiveDate**> |  | 
**status** | **String** |  | 
**bytes_in** | **i32** |  | 
**bytes_out** | **i32** |  | 
**bytes_total** | **i32** |  | 
**included_tb** | Option<**i32**> |  | 
**used_units** | **i32** |  | 
**used_tb** | **String** | Usage rounded up to six decimal places; bytes_total is exact. | 
**billable_bytes** | **i32** | Of bytes_total, the part that may be charged. | 
**billable_units** | **i32** |  | 
**billable_tb** | **String** | Billable usage rounded up to six decimal places; billable_bytes is exact. | 
**billable_from** | Option<**chrono::NaiveDate**> | First fully billable day, when one date describes the usage. May fall after the reported month. Null when all usage is billable or streams have different boundaries; use billable_bytes for the billable total. | 
**charged_tb** | **i32** |  | 
**charged_amount** | **String** |  | 
**price_per_tb** | Option<**String**> |  | 
**currency** | **String** |  | 
**unit_bytes** | **i32** |  | 
**remaining_bytes** | Option<**i32**> |  | 
**last_sample_at** | Option<**chrono::NaiveDate**> |  | 
**daily** | **Vec<std::collections::HashMap<String, serde_json::Value>>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


