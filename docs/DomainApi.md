# \DomainApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domain_domain_cancel_create**](DomainApi.md#domain_domain_cancel_create) | **POST** /api/domain/domain/{domain}/cancel/ | 
[**domain_domain_cancel_create2**](DomainApi.md#domain_domain_cancel_create2) | **POST** /api/v1/domain/domain/{domain}/cancel/ | 
[**domain_domain_check_availability_create**](DomainApi.md#domain_domain_check_availability_create) | **POST** /api/domain/domain/check-availability/ | 
[**domain_domain_check_availability_create2**](DomainApi.md#domain_domain_check_availability_create2) | **POST** /api/v1/domain/domain/check-availability/ | 
[**domain_domain_contacts_create**](DomainApi.md#domain_domain_contacts_create) | **POST** /api/domain/domain/{domain}/contacts/ | 
[**domain_domain_contacts_create2**](DomainApi.md#domain_domain_contacts_create2) | **POST** /api/v1/domain/domain/{domain}/contacts/ | 
[**domain_domain_create**](DomainApi.md#domain_domain_create) | **POST** /api/domain/domain/ | 
[**domain_domain_create2**](DomainApi.md#domain_domain_create2) | **POST** /api/v1/domain/domain/ | 
[**domain_domain_list**](DomainApi.md#domain_domain_list) | **GET** /api/domain/domain/ | 
[**domain_domain_list2**](DomainApi.md#domain_domain_list2) | **GET** /api/v1/domain/domain/ | 
[**domain_domain_nameservers_create**](DomainApi.md#domain_domain_nameservers_create) | **POST** /api/domain/domain/{domain}/nameservers/ | 
[**domain_domain_nameservers_create2**](DomainApi.md#domain_domain_nameservers_create2) | **POST** /api/v1/domain/domain/{domain}/nameservers/ | 
[**domain_domain_partial_update**](DomainApi.md#domain_domain_partial_update) | **PATCH** /api/domain/domain/{domain}/ | 
[**domain_domain_partial_update2**](DomainApi.md#domain_domain_partial_update2) | **PATCH** /api/v1/domain/domain/{domain}/ | 
[**domain_domain_renew_create**](DomainApi.md#domain_domain_renew_create) | **POST** /api/domain/domain/{domain}/renew/ | 
[**domain_domain_renew_create2**](DomainApi.md#domain_domain_renew_create2) | **POST** /api/v1/domain/domain/{domain}/renew/ | 
[**domain_domain_retrieve**](DomainApi.md#domain_domain_retrieve) | **GET** /api/domain/domain/{domain}/ | 
[**domain_domain_retrieve2**](DomainApi.md#domain_domain_retrieve2) | **GET** /api/v1/domain/domain/{domain}/ | 
[**domain_domain_transfer_ro_domain_create**](DomainApi.md#domain_domain_transfer_ro_domain_create) | **POST** /api/domain/domain/transfer-ro-domain/ | 
[**domain_domain_transfer_ro_domain_create2**](DomainApi.md#domain_domain_transfer_ro_domain_create2) | **POST** /api/v1/domain/domain/transfer-ro-domain/ | 
[**domain_domain_update**](DomainApi.md#domain_domain_update) | **PUT** /api/domain/domain/{domain}/ | 
[**domain_domain_update2**](DomainApi.md#domain_domain_update2) | **PUT** /api/v1/domain/domain/{domain}/ | 
[**domain_registrants_create**](DomainApi.md#domain_registrants_create) | **POST** /api/domain/registrants/ | 
[**domain_registrants_create2**](DomainApi.md#domain_registrants_create2) | **POST** /api/v1/domain/registrants/ | 
[**domain_registrants_destroy**](DomainApi.md#domain_registrants_destroy) | **DELETE** /api/domain/registrants/{id}/ | 
[**domain_registrants_destroy2**](DomainApi.md#domain_registrants_destroy2) | **DELETE** /api/v1/domain/registrants/{id}/ | 
[**domain_registrants_list**](DomainApi.md#domain_registrants_list) | **GET** /api/domain/registrants/ | 
[**domain_registrants_list2**](DomainApi.md#domain_registrants_list2) | **GET** /api/v1/domain/registrants/ | 
[**domain_registrants_partial_update**](DomainApi.md#domain_registrants_partial_update) | **PATCH** /api/domain/registrants/{id}/ | 
[**domain_registrants_partial_update2**](DomainApi.md#domain_registrants_partial_update2) | **PATCH** /api/v1/domain/registrants/{id}/ | 
[**domain_registrants_retrieve**](DomainApi.md#domain_registrants_retrieve) | **GET** /api/domain/registrants/{id}/ | 
[**domain_registrants_retrieve2**](DomainApi.md#domain_registrants_retrieve2) | **GET** /api/v1/domain/registrants/{id}/ | 
[**domain_registrants_update**](DomainApi.md#domain_registrants_update) | **PUT** /api/domain/registrants/{id}/ | 
[**domain_registrants_update2**](DomainApi.md#domain_registrants_update2) | **PUT** /api/v1/domain/registrants/{id}/ | 
[**domain_tld_list**](DomainApi.md#domain_tld_list) | **GET** /api/domain/tld/ | 
[**domain_tld_list2**](DomainApi.md#domain_tld_list2) | **GET** /api/v1/domain/tld/ | 
[**domain_tld_retrieve**](DomainApi.md#domain_tld_retrieve) | **GET** /api/domain/tld/{id}/ | 
[**domain_tld_retrieve2**](DomainApi.md#domain_tld_retrieve2) | **GET** /api/v1/domain/tld/{id}/ | 



## domain_domain_cancel_create

> models::DomainCancelResponse domain_domain_cancel_create(domain)


Cancel the service associated with this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |

### Return type

[**models::DomainCancelResponse**](DomainCancelResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_cancel_create2

> models::DomainCancelResponse domain_domain_cancel_create2(domain)


Cancel the service associated with this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |

### Return type

[**models::DomainCancelResponse**](DomainCancelResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_check_availability_create

> models::CheckAvailability domain_domain_check_availability_create(check_availability)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_availability** | [**CheckAvailability**](CheckAvailability.md) |  | [required] |

### Return type

[**models::CheckAvailability**](CheckAvailability.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_check_availability_create2

> models::CheckAvailability domain_domain_check_availability_create2(check_availability)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_availability** | [**CheckAvailability**](CheckAvailability.md) |  | [required] |

### Return type

[**models::CheckAvailability**](CheckAvailability.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_contacts_create

> models::ContactsUpdateResponse domain_domain_contacts_create(domain, contacts_update)


Update a contact on this domain using a saved DomainRegistrant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**contacts_update** | [**ContactsUpdate**](ContactsUpdate.md) |  | [required] |

### Return type

[**models::ContactsUpdateResponse**](ContactsUpdateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_contacts_create2

> models::ContactsUpdateResponse domain_domain_contacts_create2(domain, contacts_update)


Update a contact on this domain using a saved DomainRegistrant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**contacts_update** | [**ContactsUpdate**](ContactsUpdate.md) |  | [required] |

### Return type

[**models::ContactsUpdateResponse**](ContactsUpdateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_create

> models::DomainCreate domain_domain_create(domain_create)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_create** | [**DomainCreate**](DomainCreate.md) |  | [required] |

### Return type

[**models::DomainCreate**](DomainCreate.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_create2

> models::DomainCreate domain_domain_create2(domain_create)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_create** | [**DomainCreate**](DomainCreate.md) |  | [required] |

### Return type

[**models::DomainCreate**](DomainCreate.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_list

> models::PaginatedDomainList domain_domain_list(page)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDomainList**](PaginatedDomainList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_list2

> models::PaginatedDomainList domain_domain_list2(page)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDomainList**](PaginatedDomainList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_nameservers_create

> models::NameserversUpdateResponse domain_domain_nameservers_create(domain, nameservers_update)


Update nameservers for this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**nameservers_update** | [**NameserversUpdate**](NameserversUpdate.md) |  | [required] |

### Return type

[**models::NameserversUpdateResponse**](NameserversUpdateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_nameservers_create2

> models::NameserversUpdateResponse domain_domain_nameservers_create2(domain, nameservers_update)


Update nameservers for this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**nameservers_update** | [**NameserversUpdate**](NameserversUpdate.md) |  | [required] |

### Return type

[**models::NameserversUpdateResponse**](NameserversUpdateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_partial_update

> models::Domain domain_domain_partial_update(domain, patched_domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**patched_domain** | Option<[**PatchedDomain**](PatchedDomain.md)> |  |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_partial_update2

> models::Domain domain_domain_partial_update2(domain, patched_domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**patched_domain** | Option<[**PatchedDomain**](PatchedDomain.md)> |  |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_renew_create

> models::RenewDomain domain_domain_renew_create(domain, renew_domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**renew_domain** | [**RenewDomain**](RenewDomain.md) |  | [required] |

### Return type

[**models::RenewDomain**](RenewDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_renew_create2

> models::RenewDomain domain_domain_renew_create2(domain, renew_domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**renew_domain** | [**RenewDomain**](RenewDomain.md) |  | [required] |

### Return type

[**models::RenewDomain**](RenewDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_retrieve

> models::Domain domain_domain_retrieve(domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_retrieve2

> models::Domain domain_domain_retrieve2(domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_transfer_ro_domain_create

> models::TransferRoDomain domain_domain_transfer_ro_domain_create(transfer_ro_domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_ro_domain** | [**TransferRoDomain**](TransferRoDomain.md) |  | [required] |

### Return type

[**models::TransferRoDomain**](TransferRoDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_transfer_ro_domain_create2

> models::TransferRoDomain domain_domain_transfer_ro_domain_create2(transfer_ro_domain)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_ro_domain** | [**TransferRoDomain**](TransferRoDomain.md) |  | [required] |

### Return type

[**models::TransferRoDomain**](TransferRoDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_update

> models::Domain domain_domain_update(domain, domain2)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**domain2** | Option<[**Domain**](Domain.md)> |  |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_domain_update2

> models::Domain domain_domain_update2(domain, domain2)


Manage your domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**domain2** | Option<[**Domain**](Domain.md)> |  |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_create

> models::DomainRegistrant domain_registrants_create(domain_registrant)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_registrant** | [**DomainRegistrant**](DomainRegistrant.md) |  | [required] |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_create2

> models::DomainRegistrant domain_registrants_create2(domain_registrant)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_registrant** | [**DomainRegistrant**](DomainRegistrant.md) |  | [required] |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_destroy

> domain_registrants_destroy(id)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_destroy2

> domain_registrants_destroy2(id)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_list

> models::PaginatedDomainRegistrantList domain_registrants_list(page)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDomainRegistrantList**](PaginatedDomainRegistrantList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_list2

> models::PaginatedDomainRegistrantList domain_registrants_list2(page)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDomainRegistrantList**](PaginatedDomainRegistrantList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_partial_update

> models::DomainRegistrant domain_registrants_partial_update(id, patched_domain_registrant)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_domain_registrant** | Option<[**PatchedDomainRegistrant**](PatchedDomainRegistrant.md)> |  |  |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_partial_update2

> models::DomainRegistrant domain_registrants_partial_update2(id, patched_domain_registrant)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_domain_registrant** | Option<[**PatchedDomainRegistrant**](PatchedDomainRegistrant.md)> |  |  |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_retrieve

> models::DomainRegistrant domain_registrants_retrieve(id)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_retrieve2

> models::DomainRegistrant domain_registrants_retrieve2(id)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_update

> models::DomainRegistrant domain_registrants_update(id, domain_registrant)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**domain_registrant** | [**DomainRegistrant**](DomainRegistrant.md) |  | [required] |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_registrants_update2

> models::DomainRegistrant domain_registrants_update2(id, domain_registrant)


Manage your domain registrant views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**domain_registrant** | [**DomainRegistrant**](DomainRegistrant.md) |  | [required] |

### Return type

[**models::DomainRegistrant**](DomainRegistrant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_tld_list

> models::PaginatedTldList domain_tld_list(page)


Manage your TLDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedTldList**](PaginatedTLDList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_tld_list2

> models::PaginatedTldList domain_tld_list2(page)


Manage your TLDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedTldList**](PaginatedTLDList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_tld_retrieve

> models::Tld domain_tld_retrieve(id)


Manage your TLDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this top level domain. | [required] |

### Return type

[**models::Tld**](TLD.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_tld_retrieve2

> models::Tld domain_tld_retrieve2(id)


Manage your TLDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this top level domain. | [required] |

### Return type

[**models::Tld**](TLD.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

