# \HostingApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hosting_hosting_change_password_create**](HostingApi.md#hosting_hosting_change_password_create) | **POST** /api/hosting/hosting/{id}/change-password/ | 
[**hosting_hosting_list**](HostingApi.md#hosting_hosting_list) | **GET** /api/hosting/hosting/ | 
[**hosting_hosting_retrieve**](HostingApi.md#hosting_hosting_retrieve) | **GET** /api/hosting/hosting/{id}/ | 



## hosting_hosting_change_password_create

> models::HostingChangePasswordResponse hosting_hosting_change_password_create(id, change_password)


Change the cPanel password for this hosting service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**change_password** | [**ChangePassword**](ChangePassword.md) |  | [required] |

### Return type

[**models::HostingChangePasswordResponse**](HostingChangePasswordResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosting_hosting_list

> models::PaginatedHostingServiceList hosting_hosting_list(page)


List and manage cPanel/shared hosting services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedHostingServiceList**](PaginatedHostingServiceList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosting_hosting_retrieve

> models::HostingService hosting_hosting_retrieve(id)


List and manage cPanel/shared hosting services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::HostingService**](HostingService.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

