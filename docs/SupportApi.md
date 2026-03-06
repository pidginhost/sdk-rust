# \SupportApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**support_departments_list**](SupportApi.md#support_departments_list) | **GET** /api/support/departments/ | 
[**support_departments_list2**](SupportApi.md#support_departments_list2) | **GET** /api/v1/support/departments/ | 
[**support_tickets_close_create**](SupportApi.md#support_tickets_close_create) | **POST** /api/support/tickets/{id}/close/ | 
[**support_tickets_close_create2**](SupportApi.md#support_tickets_close_create2) | **POST** /api/v1/support/tickets/{id}/close/ | 
[**support_tickets_create**](SupportApi.md#support_tickets_create) | **POST** /api/support/tickets/ | 
[**support_tickets_create2**](SupportApi.md#support_tickets_create2) | **POST** /api/v1/support/tickets/ | 
[**support_tickets_list**](SupportApi.md#support_tickets_list) | **GET** /api/support/tickets/ | 
[**support_tickets_list2**](SupportApi.md#support_tickets_list2) | **GET** /api/v1/support/tickets/ | 
[**support_tickets_messages_attachment_retrieve**](SupportApi.md#support_tickets_messages_attachment_retrieve) | **GET** /api/support/tickets/{id}/messages/{message_id}/attachment/ | 
[**support_tickets_messages_attachment_retrieve2**](SupportApi.md#support_tickets_messages_attachment_retrieve2) | **GET** /api/v1/support/tickets/{id}/messages/{message_id}/attachment/ | 
[**support_tickets_reopen_create**](SupportApi.md#support_tickets_reopen_create) | **POST** /api/support/tickets/{id}/reopen/ | 
[**support_tickets_reopen_create2**](SupportApi.md#support_tickets_reopen_create2) | **POST** /api/v1/support/tickets/{id}/reopen/ | 
[**support_tickets_reply_create**](SupportApi.md#support_tickets_reply_create) | **POST** /api/support/tickets/{id}/reply/ | 
[**support_tickets_reply_create2**](SupportApi.md#support_tickets_reply_create2) | **POST** /api/v1/support/tickets/{id}/reply/ | 
[**support_tickets_retrieve**](SupportApi.md#support_tickets_retrieve) | **GET** /api/support/tickets/{id}/ | 
[**support_tickets_retrieve2**](SupportApi.md#support_tickets_retrieve2) | **GET** /api/v1/support/tickets/{id}/ | 



## support_departments_list

> Vec<models::Department> support_departments_list()


List available support departments.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Department>**](Department.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_departments_list2

> Vec<models::Department> support_departments_list2()


List available support departments.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Department>**](Department.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_close_create

> models::TicketCloseResponse support_tickets_close_create(id)


Close a ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TicketCloseResponse**](TicketCloseResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_close_create2

> models::TicketCloseResponse support_tickets_close_create2(id)


Close a ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TicketCloseResponse**](TicketCloseResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_create

> models::TicketDetail support_tickets_create(ticket_create)


Create a new support ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_create** | [**TicketCreate**](TicketCreate.md) |  | [required] |

### Return type

[**models::TicketDetail**](TicketDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_create2

> models::TicketDetail support_tickets_create2(ticket_create)


Create a new support ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_create** | [**TicketCreate**](TicketCreate.md) |  | [required] |

### Return type

[**models::TicketDetail**](TicketDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_list

> models::PaginatedTicketListList support_tickets_list(page)


List, create, and manage support tickets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedTicketListList**](PaginatedTicketListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_list2

> models::PaginatedTicketListList support_tickets_list2(page)


List, create, and manage support tickets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedTicketListList**](PaginatedTicketListList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_messages_attachment_retrieve

> std::path::PathBuf support_tickets_messages_attachment_retrieve(id, message_id)


Download an attachment from a ticket message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_messages_attachment_retrieve2

> std::path::PathBuf support_tickets_messages_attachment_retrieve2(id, message_id)


Download an attachment from a ticket message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_reopen_create

> models::TicketReopenResponse support_tickets_reopen_create(id)


Reopen a closed ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TicketReopenResponse**](TicketReopenResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_reopen_create2

> models::TicketReopenResponse support_tickets_reopen_create2(id)


Reopen a closed ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TicketReopenResponse**](TicketReopenResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_reply_create

> models::TicketReplyResponse support_tickets_reply_create(id, ticket_reply)


Reply to a ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**ticket_reply** | [**TicketReply**](TicketReply.md) |  | [required] |

### Return type

[**models::TicketReplyResponse**](TicketReplyResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_reply_create2

> models::TicketReplyResponse support_tickets_reply_create2(id, ticket_reply)


Reply to a ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**ticket_reply** | [**TicketReply**](TicketReply.md) |  | [required] |

### Return type

[**models::TicketReplyResponse**](TicketReplyResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_retrieve

> models::TicketDetail support_tickets_retrieve(id)


List, create, and manage support tickets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TicketDetail**](TicketDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_tickets_retrieve2

> models::TicketDetail support_tickets_retrieve2(id)


List, create, and manage support tickets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TicketDetail**](TicketDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

