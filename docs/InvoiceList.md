# InvoiceList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**number_proforma** | **String** |  | [readonly]
**number_fiscal** | **String** |  | [readonly]
**status** | [**models::InvoiceStatusEnum**](InvoiceStatusEnum.md) |  | [readonly]
**subtotal** | **String** |  | [readonly]
**vat_value** | **String** |  | [readonly]
**vat_percentage** | **i32** |  | [readonly]
**total** | **String** |  | [readonly]
**invoice_date** | **chrono::NaiveDate** |  | [readonly]
**due_date** | Option<**chrono::NaiveDate**> |  | [readonly]
**payment_date** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


