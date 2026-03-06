# \DedicatedApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dedicated_servers_list**](DedicatedApi.md#dedicated_servers_list) | **GET** /api/dedicated/servers/ | 
[**dedicated_servers_list2**](DedicatedApi.md#dedicated_servers_list2) | **GET** /api/v1/dedicated/servers/ | 
[**dedicated_servers_power_create**](DedicatedApi.md#dedicated_servers_power_create) | **POST** /api/dedicated/servers/{id}/power/ | 
[**dedicated_servers_power_create2**](DedicatedApi.md#dedicated_servers_power_create2) | **POST** /api/v1/dedicated/servers/{id}/power/ | 
[**dedicated_servers_rdns_create**](DedicatedApi.md#dedicated_servers_rdns_create) | **POST** /api/dedicated/servers/{id}/rdns/ | 
[**dedicated_servers_rdns_create2**](DedicatedApi.md#dedicated_servers_rdns_create2) | **POST** /api/v1/dedicated/servers/{id}/rdns/ | 
[**dedicated_servers_reinstall_create**](DedicatedApi.md#dedicated_servers_reinstall_create) | **POST** /api/dedicated/servers/{id}/reinstall/ | 
[**dedicated_servers_reinstall_create2**](DedicatedApi.md#dedicated_servers_reinstall_create2) | **POST** /api/v1/dedicated/servers/{id}/reinstall/ | 
[**dedicated_servers_retrieve**](DedicatedApi.md#dedicated_servers_retrieve) | **GET** /api/dedicated/servers/{id}/ | 
[**dedicated_servers_retrieve2**](DedicatedApi.md#dedicated_servers_retrieve2) | **GET** /api/v1/dedicated/servers/{id}/ | 



## dedicated_servers_list

> models::PaginatedDedicatedServerList dedicated_servers_list(page)


List and manage dedicated server services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDedicatedServerList**](PaginatedDedicatedServerList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_list2

> models::PaginatedDedicatedServerList dedicated_servers_list2(page)


List and manage dedicated server services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedDedicatedServerList**](PaginatedDedicatedServerList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_power_create

> models::PowerActionResponse dedicated_servers_power_create(id, power_action)


Execute a power management action (start, stop, restart, shutdown).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**power_action** | [**PowerAction**](PowerAction.md) |  | [required] |

### Return type

[**models::PowerActionResponse**](PowerActionResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_power_create2

> models::PowerActionResponse dedicated_servers_power_create2(id, power_action)


Execute a power management action (start, stop, restart, shutdown).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**power_action** | [**PowerAction**](PowerAction.md) |  | [required] |

### Return type

[**models::PowerActionResponse**](PowerActionResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_rdns_create

> models::RdnsUpdateResponse dedicated_servers_rdns_create(id, dedicated_rdns)


Update reverse DNS for a dedicated server IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dedicated_rdns** | [**DedicatedRdns**](DedicatedRdns.md) |  | [required] |

### Return type

[**models::RdnsUpdateResponse**](RDNSUpdateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_rdns_create2

> models::RdnsUpdateResponse dedicated_servers_rdns_create2(id, dedicated_rdns)


Update reverse DNS for a dedicated server IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dedicated_rdns** | [**DedicatedRdns**](DedicatedRdns.md) |  | [required] |

### Return type

[**models::RdnsUpdateResponse**](RDNSUpdateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_reinstall_create

> models::ReinstallResponse dedicated_servers_reinstall_create(id, reinstall)


Reinstall the dedicated server with a new operating system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**reinstall** | [**Reinstall**](Reinstall.md) |  | [required] |

### Return type

[**models::ReinstallResponse**](ReinstallResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_reinstall_create2

> models::ReinstallResponse dedicated_servers_reinstall_create2(id, reinstall)


Reinstall the dedicated server with a new operating system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**reinstall** | [**Reinstall**](Reinstall.md) |  | [required] |

### Return type

[**models::ReinstallResponse**](ReinstallResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_retrieve

> models::DedicatedServer dedicated_servers_retrieve(id)


List and manage dedicated server services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DedicatedServer**](DedicatedServer.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dedicated_servers_retrieve2

> models::DedicatedServer dedicated_servers_retrieve2(id)


List and manage dedicated server services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DedicatedServer**](DedicatedServer.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

