# \EmailApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**email_api_credentials_create**](EmailApi.md#email_api_credentials_create) | **POST** /api/email/api_credentials/ | 
[**email_api_credentials_destroy**](EmailApi.md#email_api_credentials_destroy) | **DELETE** /api/email/api_credentials/{id}/ | 
[**email_api_credentials_list**](EmailApi.md#email_api_credentials_list) | **GET** /api/email/api_credentials/ | 
[**email_api_credentials_retrieve**](EmailApi.md#email_api_credentials_retrieve) | **GET** /api/email/api_credentials/{id}/ | 
[**email_domains_create**](EmailApi.md#email_domains_create) | **POST** /api/email/domains/ | 
[**email_domains_inbound_routes_create**](EmailApi.md#email_domains_inbound_routes_create) | **POST** /api/email/domains/{domain_pk}/inbound_routes/ | 
[**email_domains_inbound_routes_list**](EmailApi.md#email_domains_inbound_routes_list) | **GET** /api/email/domains/{domain_pk}/inbound_routes/ | 
[**email_domains_list**](EmailApi.md#email_domains_list) | **GET** /api/email/domains/ | 
[**email_domains_retrieve**](EmailApi.md#email_domains_retrieve) | **GET** /api/email/domains/{id}/ | 
[**email_domains_rotate_dkim_create**](EmailApi.md#email_domains_rotate_dkim_create) | **POST** /api/email/domains/{id}/rotate_dkim/ | 
[**email_domains_toggle_inbound_create**](EmailApi.md#email_domains_toggle_inbound_create) | **POST** /api/email/domains/{id}/toggle_inbound/ | 
[**email_domains_verify_create**](EmailApi.md#email_domains_verify_create) | **POST** /api/email/domains/{id}/verify/ | 
[**email_inbound_routes_create**](EmailApi.md#email_inbound_routes_create) | **POST** /api/email/inbound_routes/ | 
[**email_inbound_routes_destroy**](EmailApi.md#email_inbound_routes_destroy) | **DELETE** /api/email/inbound_routes/{id}/ | 
[**email_inbound_routes_list**](EmailApi.md#email_inbound_routes_list) | **GET** /api/email/inbound_routes/ | 
[**email_inbound_routes_partial_update**](EmailApi.md#email_inbound_routes_partial_update) | **PATCH** /api/email/inbound_routes/{id}/ | 
[**email_inbound_routes_retrieve**](EmailApi.md#email_inbound_routes_retrieve) | **GET** /api/email/inbound_routes/{id}/ | 
[**email_messages_retrieve**](EmailApi.md#email_messages_retrieve) | **GET** /api/email/messages/{message_id}/ | 
[**email_sandbox_addresses_create**](EmailApi.md#email_sandbox_addresses_create) | **POST** /api/email/sandbox_addresses/ | 
[**email_sandbox_addresses_destroy**](EmailApi.md#email_sandbox_addresses_destroy) | **DELETE** /api/email/sandbox_addresses/{id}/ | 
[**email_sandbox_addresses_list**](EmailApi.md#email_sandbox_addresses_list) | **GET** /api/email/sandbox_addresses/ | 
[**email_sandbox_addresses_retrieve**](EmailApi.md#email_sandbox_addresses_retrieve) | **GET** /api/email/sandbox_addresses/{id}/ | 
[**email_send_create**](EmailApi.md#email_send_create) | **POST** /api/email/send/ | 
[**email_services_api_credentials_create**](EmailApi.md#email_services_api_credentials_create) | **POST** /api/email/services/{service_pk}/api_credentials/ | 
[**email_services_api_credentials_list**](EmailApi.md#email_services_api_credentials_list) | **GET** /api/email/services/{service_pk}/api_credentials/ | 
[**email_services_cancel_create**](EmailApi.md#email_services_cancel_create) | **POST** /api/email/services/{id}/cancel/ | 
[**email_services_change_tier_partial_update**](EmailApi.md#email_services_change_tier_partial_update) | **PATCH** /api/email/services/{id}/change_tier/ | 
[**email_services_create**](EmailApi.md#email_services_create) | **POST** /api/email/services/ | 
[**email_services_dedicated_ip_create**](EmailApi.md#email_services_dedicated_ip_create) | **POST** /api/email/services/{id}/dedicated_ip/ | 
[**email_services_dedicated_ip_destroy**](EmailApi.md#email_services_dedicated_ip_destroy) | **DELETE** /api/email/services/{id}/dedicated_ip/ | 
[**email_services_destroy**](EmailApi.md#email_services_destroy) | **DELETE** /api/email/services/{id}/ | 
[**email_services_domains_create**](EmailApi.md#email_services_domains_create) | **POST** /api/email/services/{service_pk}/domains/ | 
[**email_services_domains_list**](EmailApi.md#email_services_domains_list) | **GET** /api/email/services/{service_pk}/domains/ | 
[**email_services_list**](EmailApi.md#email_services_list) | **GET** /api/email/services/ | 
[**email_services_messages_retrieve**](EmailApi.md#email_services_messages_retrieve) | **GET** /api/email/services/{service_pk}/messages/ | 
[**email_services_partial_update**](EmailApi.md#email_services_partial_update) | **PATCH** /api/email/services/{id}/ | 
[**email_services_restore_create**](EmailApi.md#email_services_restore_create) | **POST** /api/email/services/{id}/restore/ | 
[**email_services_retrieve**](EmailApi.md#email_services_retrieve) | **GET** /api/email/services/{id}/ | 
[**email_services_sandbox_addresses_create**](EmailApi.md#email_services_sandbox_addresses_create) | **POST** /api/email/services/{service_pk}/sandbox_addresses/ | 
[**email_services_sandbox_addresses_list**](EmailApi.md#email_services_sandbox_addresses_list) | **GET** /api/email/services/{service_pk}/sandbox_addresses/ | 
[**email_services_smtp_credentials_create**](EmailApi.md#email_services_smtp_credentials_create) | **POST** /api/email/services/{service_pk}/smtp_credentials/ | 
[**email_services_smtp_credentials_list**](EmailApi.md#email_services_smtp_credentials_list) | **GET** /api/email/services/{service_pk}/smtp_credentials/ | 
[**email_services_stats_retrieve**](EmailApi.md#email_services_stats_retrieve) | **GET** /api/email/services/{service_pk}/stats/ | 
[**email_services_suppressions_create**](EmailApi.md#email_services_suppressions_create) | **POST** /api/email/services/{service_pk}/suppressions/ | 
[**email_services_suppressions_list**](EmailApi.md#email_services_suppressions_list) | **GET** /api/email/services/{service_pk}/suppressions/ | 
[**email_smtp_credentials_create**](EmailApi.md#email_smtp_credentials_create) | **POST** /api/email/smtp_credentials/ | 
[**email_smtp_credentials_destroy**](EmailApi.md#email_smtp_credentials_destroy) | **DELETE** /api/email/smtp_credentials/{id}/ | 
[**email_smtp_credentials_list**](EmailApi.md#email_smtp_credentials_list) | **GET** /api/email/smtp_credentials/ | 
[**email_smtp_credentials_retrieve**](EmailApi.md#email_smtp_credentials_retrieve) | **GET** /api/email/smtp_credentials/{id}/ | 
[**email_suppressions_create**](EmailApi.md#email_suppressions_create) | **POST** /api/email/suppressions/ | 
[**email_suppressions_destroy**](EmailApi.md#email_suppressions_destroy) | **DELETE** /api/email/suppressions/{id}/ | 
[**email_suppressions_list**](EmailApi.md#email_suppressions_list) | **GET** /api/email/suppressions/ | 
[**email_suppressions_retrieve**](EmailApi.md#email_suppressions_retrieve) | **GET** /api/email/suppressions/{id}/ | 



## email_api_credentials_create

> models::ApiCredential email_api_credentials_create(api_credential)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_credential** | Option<[**ApiCredential**](ApiCredential.md)> |  |  |

### Return type

[**models::ApiCredential**](ApiCredential.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_api_credentials_destroy

> email_api_credentials_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this api credential. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_api_credentials_list

> models::PaginatedApiCredentialList email_api_credentials_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedApiCredentialList**](PaginatedApiCredentialList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_api_credentials_retrieve

> models::ApiCredential email_api_credentials_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this api credential. | [required] |

### Return type

[**models::ApiCredential**](ApiCredential.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_create

> models::SendingDomain email_domains_create(domain_add)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_add** | [**DomainAdd**](DomainAdd.md) |  | [required] |

### Return type

[**models::SendingDomain**](SendingDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_inbound_routes_create

> models::InboundRoute email_domains_inbound_routes_create(domain_pk, inbound_route)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_pk** | **i32** |  | [required] |
**inbound_route** | [**InboundRoute**](InboundRoute.md) |  | [required] |

### Return type

[**models::InboundRoute**](InboundRoute.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_inbound_routes_list

> models::PaginatedInboundRouteList email_domains_inbound_routes_list(domain_pk, page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_pk** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedInboundRouteList**](PaginatedInboundRouteList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_list

> models::PaginatedSendingDomainList email_domains_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSendingDomainList**](PaginatedSendingDomainList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_retrieve

> models::SendingDomain email_domains_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this sending domain. | [required] |

### Return type

[**models::SendingDomain**](SendingDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_rotate_dkim_create

> models::SendingDomain email_domains_rotate_dkim_create(id, sending_domain)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this sending domain. | [required] |
**sending_domain** | Option<[**SendingDomain**](SendingDomain.md)> |  |  |

### Return type

[**models::SendingDomain**](SendingDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_toggle_inbound_create

> models::SendingDomain email_domains_toggle_inbound_create(id, sending_domain)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this sending domain. | [required] |
**sending_domain** | Option<[**SendingDomain**](SendingDomain.md)> |  |  |

### Return type

[**models::SendingDomain**](SendingDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_domains_verify_create

> models::SendingDomain email_domains_verify_create(id, sending_domain)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this sending domain. | [required] |
**sending_domain** | Option<[**SendingDomain**](SendingDomain.md)> |  |  |

### Return type

[**models::SendingDomain**](SendingDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_inbound_routes_create

> models::InboundRoute email_inbound_routes_create(inbound_route)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbound_route** | [**InboundRoute**](InboundRoute.md) |  | [required] |

### Return type

[**models::InboundRoute**](InboundRoute.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_inbound_routes_destroy

> email_inbound_routes_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this inbound route. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_inbound_routes_list

> models::PaginatedInboundRouteList email_inbound_routes_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedInboundRouteList**](PaginatedInboundRouteList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_inbound_routes_partial_update

> models::InboundRoute email_inbound_routes_partial_update(id, patched_inbound_route)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this inbound route. | [required] |
**patched_inbound_route** | Option<[**PatchedInboundRoute**](PatchedInboundRoute.md)> |  |  |

### Return type

[**models::InboundRoute**](InboundRoute.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_inbound_routes_retrieve

> models::InboundRoute email_inbound_routes_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this inbound route. | [required] |

### Return type

[**models::InboundRoute**](InboundRoute.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_messages_retrieve

> email_messages_retrieve(message_id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_sandbox_addresses_create

> models::SandboxAddress email_sandbox_addresses_create(sandbox_address)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sandbox_address** | [**SandboxAddress**](SandboxAddress.md) |  | [required] |

### Return type

[**models::SandboxAddress**](SandboxAddress.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_sandbox_addresses_destroy

> email_sandbox_addresses_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this sandbox verified address. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_sandbox_addresses_list

> models::PaginatedSandboxAddressList email_sandbox_addresses_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSandboxAddressList**](PaginatedSandboxAddressList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_sandbox_addresses_retrieve

> models::SandboxAddress email_sandbox_addresses_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this sandbox verified address. | [required] |

### Return type

[**models::SandboxAddress**](SandboxAddress.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_send_create

> email_send_create()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_api_credentials_create

> models::ApiCredential email_services_api_credentials_create(service_pk, api_credential)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**api_credential** | Option<[**ApiCredential**](ApiCredential.md)> |  |  |

### Return type

[**models::ApiCredential**](ApiCredential.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_api_credentials_list

> models::PaginatedApiCredentialList email_services_api_credentials_list(service_pk, page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedApiCredentialList**](PaginatedApiCredentialList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_cancel_create

> models::EmailService email_services_cancel_create(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_change_tier_partial_update

> models::EmailService email_services_change_tier_partial_update(id, patched_subscribe)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |
**patched_subscribe** | Option<[**PatchedSubscribe**](PatchedSubscribe.md)> |  |  |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_create

> models::EmailService email_services_create(subscribe)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscribe** | [**Subscribe**](Subscribe.md) |  | [required] |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_dedicated_ip_create

> models::EmailService email_services_dedicated_ip_create(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_dedicated_ip_destroy

> email_services_dedicated_ip_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_destroy

> email_services_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_domains_create

> models::SendingDomain email_services_domains_create(service_pk, domain_add)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**domain_add** | [**DomainAdd**](DomainAdd.md) |  | [required] |

### Return type

[**models::SendingDomain**](SendingDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_domains_list

> models::PaginatedSendingDomainList email_services_domains_list(service_pk, page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSendingDomainList**](PaginatedSendingDomainList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_list

> models::PaginatedEmailServiceList email_services_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedEmailServiceList**](PaginatedEmailServiceList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_messages_retrieve

> email_services_messages_retrieve(service_pk)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_partial_update

> models::EmailService email_services_partial_update(id, patched_email_service)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |
**patched_email_service** | Option<[**PatchedEmailService**](PatchedEmailService.md)> |  |  |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_restore_create

> models::EmailService email_services_restore_create(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_retrieve

> models::EmailService email_services_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this email service. | [required] |

### Return type

[**models::EmailService**](EmailService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_sandbox_addresses_create

> models::SandboxAddress email_services_sandbox_addresses_create(service_pk, sandbox_address)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**sandbox_address** | [**SandboxAddress**](SandboxAddress.md) |  | [required] |

### Return type

[**models::SandboxAddress**](SandboxAddress.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_sandbox_addresses_list

> models::PaginatedSandboxAddressList email_services_sandbox_addresses_list(service_pk, page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSandboxAddressList**](PaginatedSandboxAddressList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_smtp_credentials_create

> models::SmtpCredential email_services_smtp_credentials_create(service_pk, smtp_credential)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**smtp_credential** | Option<[**SmtpCredential**](SmtpCredential.md)> |  |  |

### Return type

[**models::SmtpCredential**](SmtpCredential.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_smtp_credentials_list

> models::PaginatedSmtpCredentialList email_services_smtp_credentials_list(service_pk, page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSmtpCredentialList**](PaginatedSmtpCredentialList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_stats_retrieve

> email_services_stats_retrieve(service_pk)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_suppressions_create

> models::SuppressionEntry email_services_suppressions_create(service_pk, suppression_entry)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**suppression_entry** | Option<[**SuppressionEntry**](SuppressionEntry.md)> |  |  |

### Return type

[**models::SuppressionEntry**](SuppressionEntry.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_services_suppressions_list

> models::PaginatedSuppressionEntryList email_services_suppressions_list(service_pk, page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_pk** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSuppressionEntryList**](PaginatedSuppressionEntryList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_smtp_credentials_create

> models::SmtpCredential email_smtp_credentials_create(smtp_credential)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smtp_credential** | Option<[**SmtpCredential**](SmtpCredential.md)> |  |  |

### Return type

[**models::SmtpCredential**](SmtpCredential.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_smtp_credentials_destroy

> email_smtp_credentials_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this smtp credential. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_smtp_credentials_list

> models::PaginatedSmtpCredentialList email_smtp_credentials_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSmtpCredentialList**](PaginatedSmtpCredentialList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_smtp_credentials_retrieve

> models::SmtpCredential email_smtp_credentials_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this smtp credential. | [required] |

### Return type

[**models::SmtpCredential**](SmtpCredential.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_suppressions_create

> models::SuppressionEntry email_suppressions_create(suppression_entry)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**suppression_entry** | Option<[**SuppressionEntry**](SuppressionEntry.md)> |  |  |

### Return type

[**models::SuppressionEntry**](SuppressionEntry.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_suppressions_destroy

> email_suppressions_destroy(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this suppression entry. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_suppressions_list

> models::PaginatedSuppressionEntryList email_suppressions_list(page)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSuppressionEntryList**](PaginatedSuppressionEntryList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## email_suppressions_retrieve

> models::SuppressionEntry email_suppressions_retrieve(id)


Require complete user profile for provisioning (create) API actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this suppression entry. | [required] |

### Return type

[**models::SuppressionEntry**](SuppressionEntry.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

