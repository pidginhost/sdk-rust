# \AccountApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_api_tokens_create**](AccountApi.md#account_api_tokens_create) | **POST** /api/account/api-tokens/ | 
[**account_api_tokens_destroy**](AccountApi.md#account_api_tokens_destroy) | **DELETE** /api/account/api-tokens/{id}/ | 
[**account_api_tokens_list**](AccountApi.md#account_api_tokens_list) | **GET** /api/account/api-tokens/ | 
[**account_companies_create**](AccountApi.md#account_companies_create) | **POST** /api/account/companies/ | 
[**account_companies_destroy**](AccountApi.md#account_companies_destroy) | **DELETE** /api/account/companies/{id}/ | 
[**account_companies_list**](AccountApi.md#account_companies_list) | **GET** /api/account/companies/ | 
[**account_companies_partial_update**](AccountApi.md#account_companies_partial_update) | **PATCH** /api/account/companies/{id}/ | 
[**account_companies_retrieve**](AccountApi.md#account_companies_retrieve) | **GET** /api/account/companies/{id}/ | 
[**account_companies_update**](AccountApi.md#account_companies_update) | **PUT** /api/account/companies/{id}/ | 
[**account_emails_list**](AccountApi.md#account_emails_list) | **GET** /api/account/emails/ | 
[**account_profile_partial_update**](AccountApi.md#account_profile_partial_update) | **PATCH** /api/account/profile | 
[**account_profile_retrieve**](AccountApi.md#account_profile_retrieve) | **GET** /api/account/profile | 
[**account_profile_update**](AccountApi.md#account_profile_update) | **PUT** /api/account/profile | 
[**account_ssh_keys_create**](AccountApi.md#account_ssh_keys_create) | **POST** /api/account/ssh-keys/ | 
[**account_ssh_keys_destroy**](AccountApi.md#account_ssh_keys_destroy) | **DELETE** /api/account/ssh-keys/{id}/ | 
[**account_ssh_keys_list**](AccountApi.md#account_ssh_keys_list) | **GET** /api/account/ssh-keys/ | 
[**account_ssh_keys_partial_update**](AccountApi.md#account_ssh_keys_partial_update) | **PATCH** /api/account/ssh-keys/{id}/ | 
[**account_ssh_keys_retrieve**](AccountApi.md#account_ssh_keys_retrieve) | **GET** /api/account/ssh-keys/{id}/ | 
[**account_ssh_keys_update**](AccountApi.md#account_ssh_keys_update) | **PUT** /api/account/ssh-keys/{id}/ | 



## account_api_tokens_create

> models::ApiTokenCreate account_api_tokens_create(api_token_create)


Manage your API tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_token_create** | [**ApiTokenCreate**](ApiTokenCreate.md) |  | [required] |

### Return type

[**models::ApiTokenCreate**](APITokenCreate.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_api_tokens_destroy

> account_api_tokens_destroy(id)


Manage your API tokens

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


## account_api_tokens_list

> models::PaginatedApiTokenListList account_api_tokens_list(page)


Manage your API tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedApiTokenListList**](PaginatedAPITokenListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_companies_create

> models::Company account_companies_create(company)


Manage your companies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company** | [**Company**](Company.md) |  | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_companies_destroy

> account_companies_destroy(id)


Manage your companies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this company. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_companies_list

> models::PaginatedCompanyList account_companies_list(page)


Manage your companies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedCompanyList**](PaginatedCompanyList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_companies_partial_update

> models::Company account_companies_partial_update(id, patched_company)


Manage your companies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this company. | [required] |
**patched_company** | Option<[**PatchedCompany**](PatchedCompany.md)> |  |  |

### Return type

[**models::Company**](Company.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_companies_retrieve

> models::Company account_companies_retrieve(id)


Manage your companies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this company. | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_companies_update

> models::Company account_companies_update(id, company)


Manage your companies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this company. | [required] |
**company** | [**Company**](Company.md) |  | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_emails_list

> models::PaginatedEmailHistoryList account_emails_list(page)


List email history for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedEmailHistoryList**](PaginatedEmailHistoryList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_profile_partial_update

> models::Profile account_profile_partial_update(patched_profile)


Manage your profile data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_profile** | Option<[**PatchedProfile**](PatchedProfile.md)> |  |  |

### Return type

[**models::Profile**](Profile.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_profile_retrieve

> models::Profile account_profile_retrieve()


Manage your profile data

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Profile**](Profile.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_profile_update

> models::Profile account_profile_update(profile)


Manage your profile data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | [**Profile**](Profile.md) |  | [required] |

### Return type

[**models::Profile**](Profile.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ssh_keys_create

> models::SshKey account_ssh_keys_create(ssh_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key** | Option<[**SshKey**](SshKey.md)> |  |  |

### Return type

[**models::SshKey**](SSHKey.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ssh_keys_destroy

> account_ssh_keys_destroy(id)


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


## account_ssh_keys_list

> models::PaginatedSshKeyList account_ssh_keys_list(page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSshKeyList**](PaginatedSSHKeyList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ssh_keys_partial_update

> models::SshKey account_ssh_keys_partial_update(id, patched_ssh_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_ssh_key** | Option<[**PatchedSshKey**](PatchedSshKey.md)> |  |  |

### Return type

[**models::SshKey**](SSHKey.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ssh_keys_retrieve

> models::SshKey account_ssh_keys_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SshKey**](SSHKey.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ssh_keys_update

> models::SshKey account_ssh_keys_update(id, ssh_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**ssh_key** | Option<[**SshKey**](SshKey.md)> |  |  |

### Return type

[**models::SshKey**](SSHKey.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

