# \BillingApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**billing_deposits_create**](BillingApi.md#billing_deposits_create) | **POST** /api/billing/deposits/ | 
[**billing_deposits_create2**](BillingApi.md#billing_deposits_create2) | **POST** /api/v1/billing/deposits/ | 
[**billing_deposits_list**](BillingApi.md#billing_deposits_list) | **GET** /api/billing/deposits/ | 
[**billing_deposits_list2**](BillingApi.md#billing_deposits_list2) | **GET** /api/v1/billing/deposits/ | 
[**billing_deposits_retrieve**](BillingApi.md#billing_deposits_retrieve) | **GET** /api/billing/deposits/{id}/ | 
[**billing_deposits_retrieve2**](BillingApi.md#billing_deposits_retrieve2) | **GET** /api/v1/billing/deposits/{id}/ | 
[**billing_funds_list**](BillingApi.md#billing_funds_list) | **GET** /api/billing/funds/ | 
[**billing_funds_list2**](BillingApi.md#billing_funds_list2) | **GET** /api/v1/billing/funds/ | 
[**billing_funds_log_list**](BillingApi.md#billing_funds_log_list) | **GET** /api/billing/funds-log/ | 
[**billing_funds_log_list2**](BillingApi.md#billing_funds_log_list2) | **GET** /api/v1/billing/funds-log/ | 
[**billing_funds_notification_settings_create**](BillingApi.md#billing_funds_notification_settings_create) | **POST** /api/billing/funds/notification-settings/ | 
[**billing_funds_notification_settings_create2**](BillingApi.md#billing_funds_notification_settings_create2) | **POST** /api/v1/billing/funds/notification-settings/ | 
[**billing_invoices_list**](BillingApi.md#billing_invoices_list) | **GET** /api/billing/invoices/ | 
[**billing_invoices_list2**](BillingApi.md#billing_invoices_list2) | **GET** /api/v1/billing/invoices/ | 
[**billing_invoices_pay_with_funds_create**](BillingApi.md#billing_invoices_pay_with_funds_create) | **POST** /api/billing/invoices/{id}/pay-with-funds/ | 
[**billing_invoices_pay_with_funds_create2**](BillingApi.md#billing_invoices_pay_with_funds_create2) | **POST** /api/v1/billing/invoices/{id}/pay-with-funds/ | 
[**billing_invoices_pdf_retrieve**](BillingApi.md#billing_invoices_pdf_retrieve) | **GET** /api/billing/invoices/{id}/pdf/ | 
[**billing_invoices_pdf_retrieve2**](BillingApi.md#billing_invoices_pdf_retrieve2) | **GET** /api/v1/billing/invoices/{id}/pdf/ | 
[**billing_invoices_retrieve**](BillingApi.md#billing_invoices_retrieve) | **GET** /api/billing/invoices/{id}/ | 
[**billing_invoices_retrieve2**](BillingApi.md#billing_invoices_retrieve2) | **GET** /api/v1/billing/invoices/{id}/ | 
[**billing_services_cancel_create**](BillingApi.md#billing_services_cancel_create) | **POST** /api/billing/services/{id}/cancel/ | 
[**billing_services_cancel_create2**](BillingApi.md#billing_services_cancel_create2) | **POST** /api/v1/billing/services/{id}/cancel/ | 
[**billing_services_change_billing_cycle_create**](BillingApi.md#billing_services_change_billing_cycle_create) | **POST** /api/billing/services/{id}/change-billing-cycle/ | 
[**billing_services_change_billing_cycle_create2**](BillingApi.md#billing_services_change_billing_cycle_create2) | **POST** /api/v1/billing/services/{id}/change-billing-cycle/ | 
[**billing_services_change_company_create**](BillingApi.md#billing_services_change_company_create) | **POST** /api/billing/services/{id}/change-company/ | 
[**billing_services_change_company_create2**](BillingApi.md#billing_services_change_company_create2) | **POST** /api/v1/billing/services/{id}/change-company/ | 
[**billing_services_list**](BillingApi.md#billing_services_list) | **GET** /api/billing/services/ | 
[**billing_services_list2**](BillingApi.md#billing_services_list2) | **GET** /api/v1/billing/services/ | 
[**billing_services_retrieve**](BillingApi.md#billing_services_retrieve) | **GET** /api/billing/services/{id}/ | 
[**billing_services_retrieve2**](BillingApi.md#billing_services_retrieve2) | **GET** /api/v1/billing/services/{id}/ | 
[**billing_services_toggle_auto_payment_create**](BillingApi.md#billing_services_toggle_auto_payment_create) | **POST** /api/billing/services/{id}/toggle-auto-payment/ | 
[**billing_services_toggle_auto_payment_create2**](BillingApi.md#billing_services_toggle_auto_payment_create2) | **POST** /api/v1/billing/services/{id}/toggle-auto-payment/ | 
[**billing_subscriptions_list**](BillingApi.md#billing_subscriptions_list) | **GET** /api/billing/subscriptions/ | 
[**billing_subscriptions_list2**](BillingApi.md#billing_subscriptions_list2) | **GET** /api/v1/billing/subscriptions/ | 
[**billing_subscriptions_retrieve**](BillingApi.md#billing_subscriptions_retrieve) | **GET** /api/billing/subscriptions/{id}/ | 
[**billing_subscriptions_retrieve2**](BillingApi.md#billing_subscriptions_retrieve2) | **GET** /api/v1/billing/subscriptions/{id}/ | 



## billing_deposits_create

> models::Deposit billing_deposits_create(deposit_create)


Create a new funds deposit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_create** | [**DepositCreate**](DepositCreate.md) |  | [required] |

### Return type

[**models::Deposit**](Deposit.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_deposits_create2

> models::Deposit billing_deposits_create2(deposit_create)


Create a new funds deposit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_create** | [**DepositCreate**](DepositCreate.md) |  | [required] |

### Return type

[**models::Deposit**](Deposit.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_deposits_list

> models::PaginatedDepositList billing_deposits_list(page)


List, create, and retrieve fund deposits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDepositList**](PaginatedDepositList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_deposits_list2

> models::PaginatedDepositList billing_deposits_list2(page)


List, create, and retrieve fund deposits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDepositList**](PaginatedDepositList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_deposits_retrieve

> models::Deposit billing_deposits_retrieve(id)


List, create, and retrieve fund deposits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Deposit**](Deposit.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_deposits_retrieve2

> models::Deposit billing_deposits_retrieve2(id)


List, create, and retrieve fund deposits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Deposit**](Deposit.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_funds_list

> Vec<models::FundsBalanceResponse> billing_funds_list()


Get current funds balance and notification settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FundsBalanceResponse>**](FundsBalanceResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_funds_list2

> Vec<models::FundsBalanceResponse> billing_funds_list2()


Get current funds balance and notification settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FundsBalanceResponse>**](FundsBalanceResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_funds_log_list

> models::PaginatedFundsLogList billing_funds_log_list(page)


List funds log entries for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedFundsLogList**](PaginatedFundsLogList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_funds_log_list2

> models::PaginatedFundsLogList billing_funds_log_list2(page)


List funds log entries for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedFundsLogList**](PaginatedFundsLogList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_funds_notification_settings_create

> models::NotificationSettingsResponse billing_funds_notification_settings_create(low_balance_settings)


Update low-balance notification settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**low_balance_settings** | [**LowBalanceSettings**](LowBalanceSettings.md) |  | [required] |

### Return type

[**models::NotificationSettingsResponse**](NotificationSettingsResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_funds_notification_settings_create2

> models::NotificationSettingsResponse billing_funds_notification_settings_create2(low_balance_settings)


Update low-balance notification settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**low_balance_settings** | [**LowBalanceSettings**](LowBalanceSettings.md) |  | [required] |

### Return type

[**models::NotificationSettingsResponse**](NotificationSettingsResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_list

> models::PaginatedInvoiceListList billing_invoices_list(page)


List and retrieve invoices. Pay with funds or download PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedInvoiceListList**](PaginatedInvoiceListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_list2

> models::PaginatedInvoiceListList billing_invoices_list2(page)


List and retrieve invoices. Pay with funds or download PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedInvoiceListList**](PaginatedInvoiceListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_pay_with_funds_create

> models::PayWithFundsResponse billing_invoices_pay_with_funds_create(id)


Pay an unpaid/overdue invoice using account funds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PayWithFundsResponse**](PayWithFundsResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_pay_with_funds_create2

> models::PayWithFundsResponse billing_invoices_pay_with_funds_create2(id)


Pay an unpaid/overdue invoice using account funds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PayWithFundsResponse**](PayWithFundsResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_pdf_retrieve

> std::path::PathBuf billing_invoices_pdf_retrieve(id)


Download invoice PDF. Use ?type=proforma or ?type=fiscal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_pdf_retrieve2

> std::path::PathBuf billing_invoices_pdf_retrieve2(id)


Download invoice PDF. Use ?type=proforma or ?type=fiscal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_retrieve

> models::InvoiceDetail billing_invoices_retrieve(id)


List and retrieve invoices. Pay with funds or download PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::InvoiceDetail**](InvoiceDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_invoices_retrieve2

> models::InvoiceDetail billing_invoices_retrieve2(id)


List and retrieve invoices. Pay with funds or download PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::InvoiceDetail**](InvoiceDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_cancel_create

> models::CancelServiceResponse billing_services_cancel_create(id)


Cancel a service. Pending services are cancelled immediately; active services are terminated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CancelServiceResponse**](CancelServiceResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_cancel_create2

> models::CancelServiceResponse billing_services_cancel_create2(id)


Cancel a service. Pending services are cancelled immediately; active services are terminated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CancelServiceResponse**](CancelServiceResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_change_billing_cycle_create

> models::ChangeBillingCycleResponse billing_services_change_billing_cycle_create(id, change_billing_cycle)


Change the billing cycle of a service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**change_billing_cycle** | [**ChangeBillingCycle**](ChangeBillingCycle.md) |  | [required] |

### Return type

[**models::ChangeBillingCycleResponse**](ChangeBillingCycleResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_change_billing_cycle_create2

> models::ChangeBillingCycleResponse billing_services_change_billing_cycle_create2(id, change_billing_cycle)


Change the billing cycle of a service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**change_billing_cycle** | [**ChangeBillingCycle**](ChangeBillingCycle.md) |  | [required] |

### Return type

[**models::ChangeBillingCycleResponse**](ChangeBillingCycleResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_change_company_create

> models::ChangeCompanyResponse billing_services_change_company_create(id, change_company)


Change the company associated with a service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**change_company** | Option<[**ChangeCompany**](ChangeCompany.md)> |  |  |

### Return type

[**models::ChangeCompanyResponse**](ChangeCompanyResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_change_company_create2

> models::ChangeCompanyResponse billing_services_change_company_create2(id, change_company)


Change the company associated with a service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**change_company** | Option<[**ChangeCompany**](ChangeCompany.md)> |  |  |

### Return type

[**models::ChangeCompanyResponse**](ChangeCompanyResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_list

> models::PaginatedServiceListList billing_services_list(page)


List and manage billing services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedServiceListList**](PaginatedServiceListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_list2

> models::PaginatedServiceListList billing_services_list2(page)


List and manage billing services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedServiceListList**](PaginatedServiceListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_retrieve

> models::ServiceList billing_services_retrieve(id)


List and manage billing services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ServiceList**](ServiceList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_retrieve2

> models::ServiceList billing_services_retrieve2(id)


List and manage billing services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ServiceList**](ServiceList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_toggle_auto_payment_create

> models::ToggleAutoPaymentResponse billing_services_toggle_auto_payment_create(id)


Toggle automatic payment for a service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToggleAutoPaymentResponse**](ToggleAutoPaymentResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_services_toggle_auto_payment_create2

> models::ToggleAutoPaymentResponse billing_services_toggle_auto_payment_create2(id)


Toggle automatic payment for a service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToggleAutoPaymentResponse**](ToggleAutoPaymentResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_subscriptions_list

> models::PaginatedSubscriptionList billing_subscriptions_list(page)


List and retrieve subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSubscriptionList**](PaginatedSubscriptionList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_subscriptions_list2

> models::PaginatedSubscriptionList billing_subscriptions_list2(page)


List and retrieve subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSubscriptionList**](PaginatedSubscriptionList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_subscriptions_retrieve

> models::Subscription billing_subscriptions_retrieve(id)


List and retrieve subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_subscriptions_retrieve2

> models::Subscription billing_subscriptions_retrieve2(id)


List and retrieve subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

