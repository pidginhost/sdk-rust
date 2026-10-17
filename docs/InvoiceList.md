# InvoiceList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**number_proforma** | **String** |  | [readonly]
**number_fiscal** | **String** |  | [readonly]
**status** | [**models::InvoiceStatusEnum**](InvoiceStatusEnum.md) |  | [readonly]
**invoice_type** | [**models::InvoiceTypeEnum**](InvoiceTypeEnum.md) |  | [readonly]
**can_pay_with_funds** | **bool** |  | [readonly]
**subtotal** | **String** |  | [readonly]
**vat_value** | **String** |  | [readonly]
**vat_percentage** | **i32** |  | [readonly]
**total** | **String** |  | [readonly]
**local_amounts** | Option<[**models::InvoiceLocalAmounts**](InvoiceLocalAmounts.md)> |  | [readonly]
**invoice_date** | **chrono::NaiveDate** |  | [readonly]
**due_date** | Option<**chrono::NaiveDate**> |  | [readonly]
**payment_date** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


