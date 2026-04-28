# \FreednsApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**freedns_dns_activate_create**](FreednsApi.md#freedns_dns_activate_create) | **POST** /api/freedns/dns/activate/ | 
[**freedns_dns_add_record_create**](FreednsApi.md#freedns_dns_add_record_create) | **POST** /api/freedns/dns/add-record/ | 
[**freedns_dns_deactivate_create**](FreednsApi.md#freedns_dns_deactivate_create) | **POST** /api/freedns/dns/deactivate/ | 
[**freedns_dns_delete_record_create**](FreednsApi.md#freedns_dns_delete_record_create) | **POST** /api/freedns/dns/delete-record/ | 
[**freedns_dns_list**](FreednsApi.md#freedns_dns_list) | **GET** /api/freedns/dns/ | 
[**freedns_dns_records_list**](FreednsApi.md#freedns_dns_records_list) | **GET** /api/freedns/dns/records/ | 



## freedns_dns_activate_create

> models::ActivateFreeDnsResponse freedns_dns_activate_create(activate_free_dns)


Activate FreeDNS for a domain. For internal domains the nameservers are changed to PidginHost NS. A default zone is created on the cPanel node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activate_free_dns** | [**ActivateFreeDns**](ActivateFreeDns.md) |  | [required] |

### Return type

[**models::ActivateFreeDnsResponse**](ActivateFreeDNSResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## freedns_dns_add_record_create

> models::DnsRecordMutateResponse freedns_dns_add_record_create(domain, source, dns_record_create)


Add or edit a DNS record. To edit an existing record, include the 'line' field with its line number. Required type-specific fields depend on 'type': A/AAAA → address; CNAME → cname; MX → preference, exchange; SRV → priority, weight, port, target; TXT → txtdata, unencoded; TYPE257 (CAA) → flag, tag, value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Domain name or PK. | [required] |
**source** | **String** | 'internal' or 'external'. | [required] |
**dns_record_create** | [**DnsRecordCreate**](DnsRecordCreate.md) |  | [required] |

### Return type

[**models::DnsRecordMutateResponse**](DNSRecordMutateResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## freedns_dns_deactivate_create

> models::DeactivateFreeDnsResponse freedns_dns_deactivate_create(deactivate_free_dns)


Deactivate FreeDNS for a domain. The DNS zone is removed from the cPanel node and, for internal domains, the original nameservers are restored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deactivate_free_dns** | [**DeactivateFreeDns**](DeactivateFreeDns.md) |  | [required] |

### Return type

[**models::DeactivateFreeDnsResponse**](DeactivateFreeDNSResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## freedns_dns_delete_record_create

> models::DeleteRecordResponse freedns_dns_delete_record_create(domain, source, delete_record)


Delete a DNS record by its line number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Domain name or PK. | [required] |
**source** | **String** | 'internal' or 'external'. | [required] |
**delete_record** | [**DeleteRecord**](DeleteRecord.md) |  | [required] |

### Return type

[**models::DeleteRecordResponse**](DeleteRecordResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## freedns_dns_list

> Vec<models::FreeDnsDomain> freedns_dns_list()


List all domains with active FreeDNS for the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FreeDnsDomain>**](FreeDNSDomain.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## freedns_dns_records_list

> Vec<models::DnsRecord> freedns_dns_records_list(domain, source)


List all DNS records for a domain with active FreeDNS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Domain name or PK. | [required] |
**source** | **String** | 'internal' or 'external'. | [required] |

### Return type

[**Vec<models::DnsRecord>**](DNSRecord.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

