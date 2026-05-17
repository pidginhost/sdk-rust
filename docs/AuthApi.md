# \AuthApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_cli_session_create**](AuthApi.md#auth_cli_session_create) | **POST** /api/auth/cli-session/ | 
[**auth_cli_session_retrieve**](AuthApi.md#auth_cli_session_retrieve) | **GET** /api/auth/cli-session/{session_id}/ | 



## auth_cli_session_create

> models::CliSessionCreateResponse auth_cli_session_create()


Create a CLI authentication session for browser-based approval

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CliSessionCreateResponse**](CLISessionCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_cli_session_retrieve

> models::CliSessionPollResponse auth_cli_session_retrieve(session_id)


Poll a CLI authentication session. Returns token when approved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::CliSessionPollResponse**](CLISessionPollResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

