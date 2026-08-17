# \CloudApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_buckets_create**](CloudApi.md#cloud_buckets_create) | **POST** /api/cloud/buckets/ | 
[**cloud_buckets_credentials_reveal_create**](CloudApi.md#cloud_buckets_credentials_reveal_create) | **POST** /api/cloud/buckets/{id}/credentials/reveal/ | 
[**cloud_buckets_credentials_rotate_create**](CloudApi.md#cloud_buckets_credentials_rotate_create) | **POST** /api/cloud/buckets/{id}/credentials/rotate/ | 
[**cloud_buckets_destroy**](CloudApi.md#cloud_buckets_destroy) | **DELETE** /api/cloud/buckets/{id}/ | 
[**cloud_buckets_list**](CloudApi.md#cloud_buckets_list) | **GET** /api/cloud/buckets/ | 
[**cloud_buckets_resize_create**](CloudApi.md#cloud_buckets_resize_create) | **POST** /api/cloud/buckets/{id}/resize/ | 
[**cloud_buckets_retrieve**](CloudApi.md#cloud_buckets_retrieve) | **GET** /api/cloud/buckets/{id}/ | 
[**cloud_buckets_visibility_create**](CloudApi.md#cloud_buckets_visibility_create) | **POST** /api/cloud/buckets/{id}/visibility/ | 
[**cloud_firewall_rules_set_create**](CloudApi.md#cloud_firewall_rules_set_create) | **POST** /api/cloud/firewall-rules-set/ | 
[**cloud_firewall_rules_set_destroy**](CloudApi.md#cloud_firewall_rules_set_destroy) | **DELETE** /api/cloud/firewall-rules-set/{id}/ | 
[**cloud_firewall_rules_set_list**](CloudApi.md#cloud_firewall_rules_set_list) | **GET** /api/cloud/firewall-rules-set/ | 
[**cloud_firewall_rules_set_partial_update**](CloudApi.md#cloud_firewall_rules_set_partial_update) | **PATCH** /api/cloud/firewall-rules-set/{id}/ | 
[**cloud_firewall_rules_set_retrieve**](CloudApi.md#cloud_firewall_rules_set_retrieve) | **GET** /api/cloud/firewall-rules-set/{id}/ | 
[**cloud_firewall_rules_set_rules_create**](CloudApi.md#cloud_firewall_rules_set_rules_create) | **POST** /api/cloud/firewall-rules-set/{rules_set_id}/rules/ | 
[**cloud_firewall_rules_set_rules_destroy**](CloudApi.md#cloud_firewall_rules_set_rules_destroy) | **DELETE** /api/cloud/firewall-rules-set/{rules_set_id}/rules/{rule_id}/ | 
[**cloud_firewall_rules_set_rules_list**](CloudApi.md#cloud_firewall_rules_set_rules_list) | **GET** /api/cloud/firewall-rules-set/{rules_set_id}/rules/ | 
[**cloud_firewall_rules_set_rules_partial_update**](CloudApi.md#cloud_firewall_rules_set_rules_partial_update) | **PATCH** /api/cloud/firewall-rules-set/{rules_set_id}/rules/{rule_id}/ | 
[**cloud_firewall_rules_set_rules_retrieve**](CloudApi.md#cloud_firewall_rules_set_rules_retrieve) | **GET** /api/cloud/firewall-rules-set/{rules_set_id}/rules/{rule_id}/ | 
[**cloud_firewall_rules_set_rules_update**](CloudApi.md#cloud_firewall_rules_set_rules_update) | **PUT** /api/cloud/firewall-rules-set/{rules_set_id}/rules/{rule_id}/ | 
[**cloud_firewall_rules_set_update**](CloudApi.md#cloud_firewall_rules_set_update) | **PUT** /api/cloud/firewall-rules-set/{id}/ | 
[**cloud_floating_ipv4_authorizations_list**](CloudApi.md#cloud_floating_ipv4_authorizations_list) | **GET** /api/cloud/floating-ipv4/{id}/authorizations/ | 
[**cloud_floating_ipv4_authorize_create**](CloudApi.md#cloud_floating_ipv4_authorize_create) | **POST** /api/cloud/floating-ipv4/{id}/authorize/ | 
[**cloud_floating_ipv4_create**](CloudApi.md#cloud_floating_ipv4_create) | **POST** /api/cloud/floating-ipv4/ | 
[**cloud_floating_ipv4_destroy**](CloudApi.md#cloud_floating_ipv4_destroy) | **DELETE** /api/cloud/floating-ipv4/{id}/ | 
[**cloud_floating_ipv4_list**](CloudApi.md#cloud_floating_ipv4_list) | **GET** /api/cloud/floating-ipv4/ | 
[**cloud_floating_ipv4_rdns_create**](CloudApi.md#cloud_floating_ipv4_rdns_create) | **POST** /api/cloud/floating-ipv4/{id}/rdns/ | 
[**cloud_floating_ipv4_rdns_retrieve**](CloudApi.md#cloud_floating_ipv4_rdns_retrieve) | **GET** /api/cloud/floating-ipv4/{id}/rdns/ | 
[**cloud_floating_ipv4_retrieve**](CloudApi.md#cloud_floating_ipv4_retrieve) | **GET** /api/cloud/floating-ipv4/{id}/ | 
[**cloud_floating_ipv4_unauthorize_create**](CloudApi.md#cloud_floating_ipv4_unauthorize_create) | **POST** /api/cloud/floating-ipv4/{id}/unauthorize/ | 
[**cloud_floating_ipv6_authorizations_list**](CloudApi.md#cloud_floating_ipv6_authorizations_list) | **GET** /api/cloud/floating-ipv6/{id}/authorizations/ | 
[**cloud_floating_ipv6_authorize_create**](CloudApi.md#cloud_floating_ipv6_authorize_create) | **POST** /api/cloud/floating-ipv6/{id}/authorize/ | 
[**cloud_floating_ipv6_create**](CloudApi.md#cloud_floating_ipv6_create) | **POST** /api/cloud/floating-ipv6/ | 
[**cloud_floating_ipv6_destroy**](CloudApi.md#cloud_floating_ipv6_destroy) | **DELETE** /api/cloud/floating-ipv6/{id}/ | 
[**cloud_floating_ipv6_list**](CloudApi.md#cloud_floating_ipv6_list) | **GET** /api/cloud/floating-ipv6/ | 
[**cloud_floating_ipv6_rdns_create**](CloudApi.md#cloud_floating_ipv6_rdns_create) | **POST** /api/cloud/floating-ipv6/{id}/rdns/ | 
[**cloud_floating_ipv6_rdns_retrieve**](CloudApi.md#cloud_floating_ipv6_rdns_retrieve) | **GET** /api/cloud/floating-ipv6/{id}/rdns/ | 
[**cloud_floating_ipv6_retrieve**](CloudApi.md#cloud_floating_ipv6_retrieve) | **GET** /api/cloud/floating-ipv6/{id}/ | 
[**cloud_floating_ipv6_unauthorize_create**](CloudApi.md#cloud_floating_ipv6_unauthorize_create) | **POST** /api/cloud/floating-ipv6/{id}/unauthorize/ | 
[**cloud_generations_list**](CloudApi.md#cloud_generations_list) | **GET** /api/cloud/generations/ | List hardware generations
[**cloud_generations_retrieve**](CloudApi.md#cloud_generations_retrieve) | **GET** /api/cloud/generations/{slug}/ | 
[**cloud_images_list**](CloudApi.md#cloud_images_list) | **GET** /api/cloud/images/ | 
[**cloud_images_retrieve**](CloudApi.md#cloud_images_retrieve) | **GET** /api/cloud/images/{id}/ | 
[**cloud_ipv4_create**](CloudApi.md#cloud_ipv4_create) | **POST** /api/cloud/ipv4/ | 
[**cloud_ipv4_destroy**](CloudApi.md#cloud_ipv4_destroy) | **DELETE** /api/cloud/ipv4/{id}/ | 
[**cloud_ipv4_detach_create**](CloudApi.md#cloud_ipv4_detach_create) | **POST** /api/cloud/ipv4/{id}/detach/ | 
[**cloud_ipv4_list**](CloudApi.md#cloud_ipv4_list) | **GET** /api/cloud/ipv4/ | 
[**cloud_ipv4_rdns_create**](CloudApi.md#cloud_ipv4_rdns_create) | **POST** /api/cloud/ipv4/{id}/rdns/ | 
[**cloud_ipv4_rdns_retrieve**](CloudApi.md#cloud_ipv4_rdns_retrieve) | **GET** /api/cloud/ipv4/{id}/rdns/ | 
[**cloud_ipv4_retrieve**](CloudApi.md#cloud_ipv4_retrieve) | **GET** /api/cloud/ipv4/{id}/ | 
[**cloud_ipv6_create**](CloudApi.md#cloud_ipv6_create) | **POST** /api/cloud/ipv6/ | 
[**cloud_ipv6_destroy**](CloudApi.md#cloud_ipv6_destroy) | **DELETE** /api/cloud/ipv6/{id}/ | 
[**cloud_ipv6_detach_create**](CloudApi.md#cloud_ipv6_detach_create) | **POST** /api/cloud/ipv6/{id}/detach/ | 
[**cloud_ipv6_list**](CloudApi.md#cloud_ipv6_list) | **GET** /api/cloud/ipv6/ | 
[**cloud_ipv6_rdns_create**](CloudApi.md#cloud_ipv6_rdns_create) | **POST** /api/cloud/ipv6/{id}/rdns/ | 
[**cloud_ipv6_rdns_retrieve**](CloudApi.md#cloud_ipv6_rdns_retrieve) | **GET** /api/cloud/ipv6/{id}/rdns/ | 
[**cloud_ipv6_retrieve**](CloudApi.md#cloud_ipv6_retrieve) | **GET** /api/cloud/ipv6/{id}/ | 
[**cloud_private_networks_add_server_create**](CloudApi.md#cloud_private_networks_add_server_create) | **POST** /api/cloud/private-networks/{id}/add-server/ | 
[**cloud_private_networks_create**](CloudApi.md#cloud_private_networks_create) | **POST** /api/cloud/private-networks/ | 
[**cloud_private_networks_destroy**](CloudApi.md#cloud_private_networks_destroy) | **DELETE** /api/cloud/private-networks/{id}/ | 
[**cloud_private_networks_list**](CloudApi.md#cloud_private_networks_list) | **GET** /api/cloud/private-networks/ | 
[**cloud_private_networks_partial_update**](CloudApi.md#cloud_private_networks_partial_update) | **PATCH** /api/cloud/private-networks/{id}/ | 
[**cloud_private_networks_remove_server_create**](CloudApi.md#cloud_private_networks_remove_server_create) | **POST** /api/cloud/private-networks/{id}/remove-server/ | 
[**cloud_private_networks_retrieve**](CloudApi.md#cloud_private_networks_retrieve) | **GET** /api/cloud/private-networks/{id}/ | 
[**cloud_private_networks_update**](CloudApi.md#cloud_private_networks_update) | **PUT** /api/cloud/private-networks/{id}/ | 
[**cloud_server_packages_by_generation_retrieve**](CloudApi.md#cloud_server_packages_by_generation_retrieve) | **GET** /api/cloud/server-packages/by-generation/ | 
[**cloud_server_packages_list**](CloudApi.md#cloud_server_packages_list) | **GET** /api/cloud/server-packages/ | 
[**cloud_server_packages_retrieve**](CloudApi.md#cloud_server_packages_retrieve) | **GET** /api/cloud/server-packages/{id}/ | 
[**cloud_servers_activity_retrieve**](CloudApi.md#cloud_servers_activity_retrieve) | **GET** /api/cloud/servers/{id}/activity/ | 
[**cloud_servers_attach_ipv4_create**](CloudApi.md#cloud_servers_attach_ipv4_create) | **POST** /api/cloud/servers/{id}/attach-ipv4/ | 
[**cloud_servers_attach_ipv6_create**](CloudApi.md#cloud_servers_attach_ipv6_create) | **POST** /api/cloud/servers/{id}/attach-ipv6/ | 
[**cloud_servers_boot_isos_list**](CloudApi.md#cloud_servers_boot_isos_list) | **GET** /api/cloud/servers/{id}/boot-isos/ | 
[**cloud_servers_console_create**](CloudApi.md#cloud_servers_console_create) | **POST** /api/cloud/servers/{id}/console/ | 
[**cloud_servers_create**](CloudApi.md#cloud_servers_create) | **POST** /api/cloud/servers/ | 
[**cloud_servers_destroy**](CloudApi.md#cloud_servers_destroy) | **DELETE** /api/cloud/servers/{id}/ | 
[**cloud_servers_destroy_protection_create**](CloudApi.md#cloud_servers_destroy_protection_create) | **POST** /api/cloud/servers/{id}/destroy-protection/ | 
[**cloud_servers_detach_ipv4_create**](CloudApi.md#cloud_servers_detach_ipv4_create) | **POST** /api/cloud/servers/{id}/detach-ipv4/ | 
[**cloud_servers_detach_ipv6_create**](CloudApi.md#cloud_servers_detach_ipv6_create) | **POST** /api/cloud/servers/{id}/detach-ipv6/ | 
[**cloud_servers_list**](CloudApi.md#cloud_servers_list) | **GET** /api/cloud/servers/ | 
[**cloud_servers_modify_package_create**](CloudApi.md#cloud_servers_modify_package_create) | **POST** /api/cloud/servers/{id}/modify-package/ | 
[**cloud_servers_partial_update**](CloudApi.md#cloud_servers_partial_update) | **PATCH** /api/cloud/servers/{id}/ | 
[**cloud_servers_power_management_create**](CloudApi.md#cloud_servers_power_management_create) | **POST** /api/cloud/servers/{id}/power-management/ | 
[**cloud_servers_power_management_retrieve**](CloudApi.md#cloud_servers_power_management_retrieve) | **GET** /api/cloud/servers/{id}/power-management/ | 
[**cloud_servers_public_interface_create**](CloudApi.md#cloud_servers_public_interface_create) | **POST** /api/cloud/servers/{id}/public-interface/ | 
[**cloud_servers_public_interface_destroy**](CloudApi.md#cloud_servers_public_interface_destroy) | **DELETE** /api/cloud/servers/{id}/public-interface/ | 
[**cloud_servers_public_interface_retrieve**](CloudApi.md#cloud_servers_public_interface_retrieve) | **GET** /api/cloud/servers/{id}/public-interface/ | 
[**cloud_servers_rescue_enter_create**](CloudApi.md#cloud_servers_rescue_enter_create) | **POST** /api/cloud/servers/{id}/rescue/enter/ | 
[**cloud_servers_rescue_exit_create**](CloudApi.md#cloud_servers_rescue_exit_create) | **POST** /api/cloud/servers/{id}/rescue/exit/ | 
[**cloud_servers_retrieve**](CloudApi.md#cloud_servers_retrieve) | **GET** /api/cloud/servers/{id}/ | 
[**cloud_servers_retry_provision_create**](CloudApi.md#cloud_servers_retry_provision_create) | **POST** /api/cloud/servers/{id}/retry-provision/ | 
[**cloud_servers_snapshots_create**](CloudApi.md#cloud_servers_snapshots_create) | **POST** /api/cloud/servers/{id}/snapshots/ | 
[**cloud_servers_snapshots_destroy**](CloudApi.md#cloud_servers_snapshots_destroy) | **DELETE** /api/cloud/servers/{id}/snapshots/{snapshot_name}/ | 
[**cloud_servers_snapshots_list**](CloudApi.md#cloud_servers_snapshots_list) | **GET** /api/cloud/servers/{id}/snapshots/ | 
[**cloud_servers_snapshots_rollback_create**](CloudApi.md#cloud_servers_snapshots_rollback_create) | **POST** /api/cloud/servers/{id}/snapshots/{snapshot_name}/rollback/ | 
[**cloud_servers_update**](CloudApi.md#cloud_servers_update) | **PUT** /api/cloud/servers/{id}/ | 
[**cloud_servers_usage_retrieve**](CloudApi.md#cloud_servers_usage_retrieve) | **GET** /api/cloud/servers/{id}/usage/ | 
[**cloud_servers_volumes_create**](CloudApi.md#cloud_servers_volumes_create) | **POST** /api/cloud/servers/{server_id}/volumes/ | 
[**cloud_servers_volumes_destroy**](CloudApi.md#cloud_servers_volumes_destroy) | **DELETE** /api/cloud/servers/{server_id}/volumes/{volume_id}/ | 
[**cloud_servers_volumes_list**](CloudApi.md#cloud_servers_volumes_list) | **GET** /api/cloud/servers/{server_id}/volumes/ | 
[**cloud_servers_volumes_partial_update**](CloudApi.md#cloud_servers_volumes_partial_update) | **PATCH** /api/cloud/servers/{server_id}/volumes/{volume_id}/ | 
[**cloud_servers_volumes_retrieve**](CloudApi.md#cloud_servers_volumes_retrieve) | **GET** /api/cloud/servers/{server_id}/volumes/{volume_id}/ | 
[**cloud_servers_volumes_update**](CloudApi.md#cloud_servers_volumes_update) | **PUT** /api/cloud/servers/{server_id}/volumes/{volume_id}/ | 
[**cloud_storage_products_list**](CloudApi.md#cloud_storage_products_list) | **GET** /api/cloud/storage-products/ | 
[**cloud_storage_products_retrieve**](CloudApi.md#cloud_storage_products_retrieve) | **GET** /api/cloud/storage-products/{id}/ | 
[**cloud_volumes_attach_create**](CloudApi.md#cloud_volumes_attach_create) | **POST** /api/cloud/volumes/{id}/attach/ | 
[**cloud_volumes_destroy**](CloudApi.md#cloud_volumes_destroy) | **DELETE** /api/cloud/volumes/{id}/ | 
[**cloud_volumes_detach_create**](CloudApi.md#cloud_volumes_detach_create) | **POST** /api/cloud/volumes/{id}/detach/ | 
[**cloud_volumes_list**](CloudApi.md#cloud_volumes_list) | **GET** /api/cloud/volumes/ | 
[**cloud_volumes_partial_update**](CloudApi.md#cloud_volumes_partial_update) | **PATCH** /api/cloud/volumes/{id}/ | 
[**cloud_volumes_retrieve**](CloudApi.md#cloud_volumes_retrieve) | **GET** /api/cloud/volumes/{id}/ | 
[**cloud_volumes_update**](CloudApi.md#cloud_volumes_update) | **PUT** /api/cloud/volumes/{id}/ | 



## cloud_buckets_create

> models::Bucket cloud_buckets_create(bucket)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket** | Option<[**Bucket**](Bucket.md)> |  |  |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_credentials_reveal_create

> models::Bucket cloud_buckets_credentials_reveal_create(id, bucket)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**bucket** | Option<[**Bucket**](Bucket.md)> |  |  |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_credentials_rotate_create

> models::Bucket cloud_buckets_credentials_rotate_create(id, bucket)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**bucket** | Option<[**Bucket**](Bucket.md)> |  |  |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_destroy

> cloud_buckets_destroy(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_list

> Vec<models::Bucket> cloud_buckets_list()


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Bucket>**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_resize_create

> models::Bucket cloud_buckets_resize_create(id, bucket)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**bucket** | Option<[**Bucket**](Bucket.md)> |  |  |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_retrieve

> models::Bucket cloud_buckets_retrieve(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_visibility_create

> models::Bucket cloud_buckets_visibility_create(id, bucket)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**bucket** | Option<[**Bucket**](Bucket.md)> |  |  |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_create

> models::FirewallRulesSet cloud_firewall_rules_set_create(firewall_rules_set)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_rules_set** | [**FirewallRulesSet**](FirewallRulesSet.md) |  | [required] |

### Return type

[**models::FirewallRulesSet**](FirewallRulesSet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_destroy

> cloud_firewall_rules_set_destroy(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this firewall rules set. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_list

> Vec<models::FirewallRulesSet> cloud_firewall_rules_set_list()


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FirewallRulesSet>**](FirewallRulesSet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_partial_update

> models::FirewallRulesSet cloud_firewall_rules_set_partial_update(id, patched_firewall_rules_set)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this firewall rules set. | [required] |
**patched_firewall_rules_set** | Option<[**PatchedFirewallRulesSet**](PatchedFirewallRulesSet.md)> |  |  |

### Return type

[**models::FirewallRulesSet**](FirewallRulesSet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_retrieve

> models::FirewallRulesSet cloud_firewall_rules_set_retrieve(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this firewall rules set. | [required] |

### Return type

[**models::FirewallRulesSet**](FirewallRulesSet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_rules_create

> models::FirewallRule cloud_firewall_rules_set_rules_create(rules_set_id, firewall_rule)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rules_set_id** | **String** |  | [required] |
**firewall_rule** | [**FirewallRule**](FirewallRule.md) |  | [required] |

### Return type

[**models::FirewallRule**](FirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_rules_destroy

> cloud_firewall_rules_set_rules_destroy(rule_id, rules_set_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**rules_set_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_rules_list

> Vec<models::FirewallRule> cloud_firewall_rules_set_rules_list(rules_set_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rules_set_id** | **String** |  | [required] |

### Return type

[**Vec<models::FirewallRule>**](FirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_rules_partial_update

> models::FirewallRule cloud_firewall_rules_set_rules_partial_update(rule_id, rules_set_id, patched_firewall_rule)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**rules_set_id** | **String** |  | [required] |
**patched_firewall_rule** | Option<[**PatchedFirewallRule**](PatchedFirewallRule.md)> |  |  |

### Return type

[**models::FirewallRule**](FirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_rules_retrieve

> models::FirewallRule cloud_firewall_rules_set_rules_retrieve(rule_id, rules_set_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**rules_set_id** | **String** |  | [required] |

### Return type

[**models::FirewallRule**](FirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_rules_update

> models::FirewallRule cloud_firewall_rules_set_rules_update(rule_id, rules_set_id, firewall_rule)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**rules_set_id** | **String** |  | [required] |
**firewall_rule** | [**FirewallRule**](FirewallRule.md) |  | [required] |

### Return type

[**models::FirewallRule**](FirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_update

> models::FirewallRulesSet cloud_firewall_rules_set_update(id, firewall_rules_set)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this firewall rules set. | [required] |
**firewall_rules_set** | [**FirewallRulesSet**](FirewallRulesSet.md) |  | [required] |

### Return type

[**models::FirewallRulesSet**](FirewallRulesSet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_authorizations_list

> models::PaginatedFloatingIpAuthorizationList cloud_floating_ipv4_authorizations_list(id, page)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedFloatingIpAuthorizationList**](PaginatedFloatingIPAuthorizationList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_authorize_create

> models::FloatingIpv4AuthorizeResponse cloud_floating_ipv4_authorize_create(id, floating_ip_authorize_request)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |
**floating_ip_authorize_request** | [**FloatingIpAuthorizeRequest**](FloatingIpAuthorizeRequest.md) |  | [required] |

### Return type

[**models::FloatingIpv4AuthorizeResponse**](FloatingIPv4AuthorizeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_create

> models::FloatingIpv4 cloud_floating_ipv4_create(floating_ipv4_create)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ipv4_create** | Option<[**FloatingIpv4Create**](FloatingIpv4Create.md)> |  |  |

### Return type

[**models::FloatingIpv4**](FloatingIPv4.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_destroy

> cloud_floating_ipv4_destroy(id)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_list

> models::PaginatedFloatingIpv4List cloud_floating_ipv4_list(page)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedFloatingIpv4List**](PaginatedFloatingIPv4List.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_rdns_create

> models::ReverseDns cloud_floating_ipv4_rdns_create(id, reverse_dns)


Get or update reverse DNS (PTR) for the IPv4 address wrapped by this floating IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |
**reverse_dns** | [**ReverseDns**](ReverseDns.md) |  | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_rdns_retrieve

> models::ReverseDns cloud_floating_ipv4_rdns_retrieve(id)


Get or update reverse DNS (PTR) for the IPv4 address wrapped by this floating IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_retrieve

> models::FloatingIpv4 cloud_floating_ipv4_retrieve(id)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |

### Return type

[**models::FloatingIpv4**](FloatingIPv4.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv4_unauthorize_create

> models::FloatingIpv4UnauthorizeResponse cloud_floating_ipv4_unauthorize_create(id, floating_ip_authorize_request)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |
**floating_ip_authorize_request** | [**FloatingIpAuthorizeRequest**](FloatingIpAuthorizeRequest.md) |  | [required] |

### Return type

[**models::FloatingIpv4UnauthorizeResponse**](FloatingIPv4UnauthorizeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_authorizations_list

> models::PaginatedFloatingIpAuthorizationList cloud_floating_ipv6_authorizations_list(id, page)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedFloatingIpAuthorizationList**](PaginatedFloatingIPAuthorizationList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_authorize_create

> models::FloatingIpv6AuthorizeResponse cloud_floating_ipv6_authorize_create(id, floating_ip_authorize_request)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |
**floating_ip_authorize_request** | [**FloatingIpAuthorizeRequest**](FloatingIpAuthorizeRequest.md) |  | [required] |

### Return type

[**models::FloatingIpv6AuthorizeResponse**](FloatingIPv6AuthorizeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_create

> models::FloatingIpv6 cloud_floating_ipv6_create(floating_ipv6_create)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ipv6_create** | Option<[**FloatingIpv6Create**](FloatingIpv6Create.md)> |  |  |

### Return type

[**models::FloatingIpv6**](FloatingIPv6.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_destroy

> cloud_floating_ipv6_destroy(id)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_list

> models::PaginatedFloatingIpv6List cloud_floating_ipv6_list(page)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedFloatingIpv6List**](PaginatedFloatingIPv6List.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_rdns_create

> models::ReverseDns cloud_floating_ipv6_rdns_create(id, reverse_dns)


Get or update reverse DNS (PTR) for the IPv6 address wrapped by this floating IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |
**reverse_dns** | [**ReverseDns**](ReverseDns.md) |  | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_rdns_retrieve

> models::ReverseDns cloud_floating_ipv6_rdns_retrieve(id)


Get or update reverse DNS (PTR) for the IPv6 address wrapped by this floating IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_retrieve

> models::FloatingIpv6 cloud_floating_ipv6_retrieve(id)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |

### Return type

[**models::FloatingIpv6**](FloatingIPv6.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_floating_ipv6_unauthorize_create

> models::FloatingIpv6UnauthorizeResponse cloud_floating_ipv6_unauthorize_create(id, floating_ip_authorize_request)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |
**floating_ip_authorize_request** | [**FloatingIpAuthorizeRequest**](FloatingIpAuthorizeRequest.md) |  | [required] |

### Return type

[**models::FloatingIpv6UnauthorizeResponse**](FloatingIPv6UnauthorizeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_generations_list

> Vec<models::HardwareGeneration> cloud_generations_list()
List hardware generations

Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::HardwareGeneration>**](HardwareGeneration.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_generations_retrieve

> models::HardwareGeneration cloud_generations_retrieve(slug)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::HardwareGeneration**](HardwareGeneration.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_images_list

> models::PaginatedOsImageList cloud_images_list(page)


List of available OS images

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedOsImageList**](PaginatedOSImageList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_images_retrieve

> models::OsImage cloud_images_retrieve(id)


List of available OS images

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this operating system. | [required] |

### Return type

[**models::OsImage**](OSImage.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_create

> models::PublicIpv4 cloud_ipv4_create(public_ipv4)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_ipv4** | Option<[**PublicIpv4**](PublicIpv4.md)> |  |  |

### Return type

[**models::PublicIpv4**](PublicIPv4.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_destroy

> cloud_ipv4_destroy(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_detach_create

> models::DetachIpv4Response cloud_ipv4_detach_create(id, public_ipv4)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |
**public_ipv4** | Option<[**PublicIpv4**](PublicIpv4.md)> |  |  |

### Return type

[**models::DetachIpv4Response**](DetachIPv4Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_list

> models::PaginatedPublicIpv4List cloud_ipv4_list(page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedPublicIpv4List**](PaginatedPublicIPv4List.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_rdns_create

> models::ReverseDns cloud_ipv4_rdns_create(id, reverse_dns)


Get or update reverse DNS (PTR) for this IPv4 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |
**reverse_dns** | [**ReverseDns**](ReverseDns.md) |  | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_rdns_retrieve

> models::ReverseDns cloud_ipv4_rdns_retrieve(id)


Get or update reverse DNS (PTR) for this IPv4 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv4_retrieve

> models::PublicIpv4 cloud_ipv4_retrieve(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |

### Return type

[**models::PublicIpv4**](PublicIPv4.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_create

> models::PublicIpv6 cloud_ipv6_create(public_ipv6)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_ipv6** | Option<[**PublicIpv6**](PublicIpv6.md)> |  |  |

### Return type

[**models::PublicIpv6**](PublicIPv6.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_destroy

> cloud_ipv6_destroy(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_detach_create

> models::DetachIpv6Response cloud_ipv6_detach_create(id, public_ipv6)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |
**public_ipv6** | Option<[**PublicIpv6**](PublicIpv6.md)> |  |  |

### Return type

[**models::DetachIpv6Response**](DetachIPv6Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_list

> models::PaginatedPublicIpv6List cloud_ipv6_list(page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedPublicIpv6List**](PaginatedPublicIPv6List.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_rdns_create

> models::ReverseDns cloud_ipv6_rdns_create(id, reverse_dns)


Get or update reverse DNS (PTR) for this IPv6 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |
**reverse_dns** | [**ReverseDns**](ReverseDns.md) |  | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_rdns_retrieve

> models::ReverseDns cloud_ipv6_rdns_retrieve(id)


Get or update reverse DNS (PTR) for this IPv6 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |

### Return type

[**models::ReverseDns**](ReverseDNS.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_ipv6_retrieve

> models::PublicIpv6 cloud_ipv6_retrieve(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |

### Return type

[**models::PublicIpv6**](PublicIPv6.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_add_server_create

> models::AddServerResponse cloud_private_networks_add_server_create(id, private_network_add_host)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**private_network_add_host** | [**PrivateNetworkAddHost**](PrivateNetworkAddHost.md) |  | [required] |

### Return type

[**models::AddServerResponse**](AddServerResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_create

> models::PrivateNetwork cloud_private_networks_create(private_network)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_network** | [**PrivateNetwork**](PrivateNetwork.md) |  | [required] |

### Return type

[**models::PrivateNetwork**](PrivateNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_destroy

> cloud_private_networks_destroy(id)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_list

> models::PaginatedPrivateNetworkList cloud_private_networks_list(page)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedPrivateNetworkList**](PaginatedPrivateNetworkList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_partial_update

> models::PrivateNetwork cloud_private_networks_partial_update(id, patched_private_network)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**patched_private_network** | Option<[**PatchedPrivateNetwork**](PatchedPrivateNetwork.md)> |  |  |

### Return type

[**models::PrivateNetwork**](PrivateNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_remove_server_create

> models::RemoveServerResponse cloud_private_networks_remove_server_create(id, private_network_remove_host)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**private_network_remove_host** | [**PrivateNetworkRemoveHost**](PrivateNetworkRemoveHost.md) |  | [required] |

### Return type

[**models::RemoveServerResponse**](RemoveServerResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_retrieve

> models::PrivateNetwork cloud_private_networks_retrieve(id)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |

### Return type

[**models::PrivateNetwork**](PrivateNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_update

> models::PrivateNetwork cloud_private_networks_update(id, private_network)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**private_network** | [**PrivateNetwork**](PrivateNetwork.md) |  | [required] |

### Return type

[**models::PrivateNetwork**](PrivateNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_server_packages_by_generation_retrieve

> models::ServerProduct cloud_server_packages_by_generation_retrieve()


List of available server products

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerProduct**](ServerProduct.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_server_packages_list

> models::PaginatedServerProductList cloud_server_packages_list(generation, page)


List of available server products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generation** | Option<**String**> | Filter packages available on the given hardware generation (slug). Excludes free-tier-only packages when the generation is not free-tier eligible. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedServerProductList**](PaginatedServerProductList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_server_packages_retrieve

> models::ServerProduct cloud_server_packages_retrieve(id)


List of available server products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this metered product. | [required] |

### Return type

[**models::ServerProduct**](ServerProduct.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_activity_retrieve

> models::ActivityLogResponse cloud_servers_activity_retrieve(id)


Get activity log for a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::ActivityLogResponse**](ActivityLogResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_attach_ipv4_create

> models::AttachIpv4Response cloud_servers_attach_ipv4_create(id, attach_ipv4_request)


Attach IPv4 address to server. The first attach lands on the primary NIC; subsequent attaches add a new secondary NIC carrying just that IPv4. The address is written to the machine's network configuration, which the guest OS only reads while booting: a running server answers `reboot_required: true` and stays unreachable on that address until it is restarted. Send `reboot: true` to have the restart issued here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**attach_ipv4_request** | [**AttachIpv4Request**](AttachIpv4Request.md) |  | [required] |

### Return type

[**models::AttachIpv4Response**](AttachIPv4Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_attach_ipv6_create

> models::AttachIpv6Response cloud_servers_attach_ipv6_create(id, attach_ipv6_request)


Attach IPv6 address to server. Like IPv4, the address is written to the machine's network configuration and the guest OS reads it while booting: a running server answers `reboot_required: true` until it is restarted. Send `reboot: true` to have the restart issued here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**attach_ipv6_request** | [**AttachIpv6Request**](AttachIpv6Request.md) |  | [required] |

### Return type

[**models::AttachIpv6Response**](AttachIPv6Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_boot_isos_list

> models::PaginatedBootIsoList cloud_servers_boot_isos_list(id, page)


List the ISO catalog entries visible to this user and their package compatibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedBootIsoList**](PaginatedBootISOList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_console_create

> models::ConsoleToken cloud_servers_console_create(id)


Get a VNC console token for browser-based access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::ConsoleToken**](ConsoleToken.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_create

> models::ServerAddResponse cloud_servers_create(server_add)


Create new server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_add** | [**ServerAdd**](ServerAdd.md) |  | [required] |

### Return type

[**models::ServerAddResponse**](ServerAddResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_destroy

> cloud_servers_destroy(id)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_destroy_protection_create

> models::DestroyProtectionResponse cloud_servers_destroy_protection_create(id, destroy_protection)


Enable or disable destroy protection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**destroy_protection** | [**DestroyProtection**](DestroyProtection.md) |  | [required] |

### Return type

[**models::DestroyProtectionResponse**](DestroyProtectionResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_detach_ipv4_create

> models::ServerDetachIpv4Response cloud_servers_detach_ipv4_create(id, ipv4)


Detach IPv4 from server. Without `ipv4`, the primary NIC's IPv4 is detached. Pass `ipv4=<id|slug>` to target a specific attached address (required when the server has more than one IPv4).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**ipv4** | Option<**String**> | ID or slug of the IPv4 address to detach. |  |

### Return type

[**models::ServerDetachIpv4Response**](ServerDetachIPv4Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_detach_ipv6_create

> models::DetachIpv6 cloud_servers_detach_ipv6_create(id)


Detach IPv6 from server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::DetachIpv6**](DetachIPv6.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_list

> models::PaginatedServerList cloud_servers_list(page)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedServerList**](PaginatedServerList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_modify_package_create

> models::ServerUpgradeResponse cloud_servers_modify_package_create(id, server_product_upgrade)


Modify server package: downgrade available only for packages with the same disk size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**server_product_upgrade** | [**ServerProductUpgrade**](ServerProductUpgrade.md) |  | [required] |

### Return type

[**models::ServerUpgradeResponse**](ServerUpgradeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_partial_update

> models::ServerDetail cloud_servers_partial_update(id, patched_server_detail)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**patched_server_detail** | Option<[**PatchedServerDetail**](PatchedServerDetail.md)> |  |  |

### Return type

[**models::ServerDetail**](ServerDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_power_management_create

> models::PowerManagement cloud_servers_power_management_create(id, power_management_request)


Server power management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**power_management_request** | [**PowerManagementRequest**](PowerManagementRequest.md) |  | [required] |

### Return type

[**models::PowerManagement**](PowerManagement.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_power_management_retrieve

> models::PowerManagement cloud_servers_power_management_retrieve(id)


Server power management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::PowerManagement**](PowerManagement.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_public_interface_create

> models::PublicInterface cloud_servers_public_interface_create(id, public_interface)


Public interface

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**public_interface** | Option<[**PublicInterface**](PublicInterface.md)> |  |  |

### Return type

[**models::PublicInterface**](PublicInterface.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_public_interface_destroy

> cloud_servers_public_interface_destroy(id)


Public interface

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_public_interface_retrieve

> models::PublicInterface cloud_servers_public_interface_retrieve(id)


Public interface

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::PublicInterface**](PublicInterface.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_rescue_enter_create

> models::RescueEnterQueued cloud_servers_rescue_enter_create(id, iso_boot_request)


Boot the server from the default rescue image or a catalog ISO. The server is powered off first; use the console to complete the repair or installation. Not available for HA servers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**iso_boot_request** | Option<[**IsoBootRequest**](IsoBootRequest.md)> |  |  |

### Return type

[**models::RescueEnterQueued**](RescueEnterQueued.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_rescue_exit_create

> models::RescueExitQueued cloud_servers_rescue_exit_create(id)


Exit rescue mode: detach the rescue ISO, restore the original boot order, and boot normally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::RescueExitQueued**](RescueExitQueued.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_retrieve

> models::ServerDetail cloud_servers_retrieve(id)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::ServerDetail**](ServerDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_retry_provision_create

> models::RetryProvision cloud_servers_retry_provision_create(id)


Retry provision in case of a failed server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::RetryProvision**](RetryProvision.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_snapshots_create

> models::PaginatedSnapshotList cloud_servers_snapshots_create(id, snapshot_create, page)


List snapshots for this server or queue a new snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**snapshot_create** | [**SnapshotCreate**](SnapshotCreate.md) |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSnapshotList**](PaginatedSnapshotList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_snapshots_destroy

> models::SnapshotDeleteQueued cloud_servers_snapshots_destroy(id, snapshot_name)


Delete a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**snapshot_name** | **String** |  | [required] |

### Return type

[**models::SnapshotDeleteQueued**](SnapshotDeleteQueued.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_snapshots_list

> models::PaginatedSnapshotList cloud_servers_snapshots_list(id, page)


List snapshots for this server or queue a new snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedSnapshotList**](PaginatedSnapshotList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_snapshots_rollback_create

> models::SnapshotRollbackQueued cloud_servers_snapshots_rollback_create(id, snapshot_name)


Rollback the server to a specific snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**snapshot_name** | **String** |  | [required] |

### Return type

[**models::SnapshotRollbackQueued**](SnapshotRollbackQueued.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_update

> models::ServerDetail cloud_servers_update(id, server_detail)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**server_detail** | Option<[**ServerDetail**](ServerDetail.md)> |  |  |

### Return type

[**models::ServerDetail**](ServerDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_usage_retrieve

> models::ServerUsageResponse cloud_servers_usage_retrieve(id)


Get current resource usage for a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::ServerUsageResponse**](ServerUsageResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_volumes_create

> models::Volume cloud_servers_volumes_create(server_id, volume)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume** | [**Volume**](Volume.md) |  | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_volumes_destroy

> cloud_servers_volumes_destroy(server_id, volume_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_volumes_list

> Vec<models::Volume> cloud_servers_volumes_list(server_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |

### Return type

[**Vec<models::Volume>**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_volumes_partial_update

> models::Volume cloud_servers_volumes_partial_update(server_id, volume_id, patched_volume)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_id** | **String** |  | [required] |
**patched_volume** | Option<[**PatchedVolume**](PatchedVolume.md)> |  |  |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_volumes_retrieve

> models::Volume cloud_servers_volumes_retrieve(server_id, volume_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_id** | **String** |  | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_volumes_update

> models::Volume cloud_servers_volumes_update(server_id, volume_id, volume)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_id** | **String** |  | [required] |
**volume** | [**Volume**](Volume.md) |  | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_storage_products_list

> models::PaginatedStorageProductList cloud_storage_products_list(page)


List of available storage products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedStorageProductList**](PaginatedStorageProductList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_storage_products_retrieve

> models::StorageProduct cloud_storage_products_retrieve(id)


List of available storage products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this metered product. | [required] |

### Return type

[**models::StorageProduct**](StorageProduct.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_attach_create

> models::AttachVolume cloud_volumes_attach_create(id, attach_volume)


Attach existing volume to a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**attach_volume** | [**AttachVolume**](AttachVolume.md) |  | [required] |

### Return type

[**models::AttachVolume**](AttachVolume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_destroy

> cloud_volumes_destroy(id)


Volumes management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_detach_create

> models::DetachVolume cloud_volumes_detach_create(id, volume)


Detach volume from server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**volume** | [**Volume**](Volume.md) |  | [required] |

### Return type

[**models::DetachVolume**](DetachVolume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_list

> Vec<models::Volume> cloud_volumes_list()


Volumes management

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Volume>**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_partial_update

> models::Volume cloud_volumes_partial_update(id, patched_volume)


Volumes management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**patched_volume** | Option<[**PatchedVolume**](PatchedVolume.md)> |  |  |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_retrieve

> models::Volume cloud_volumes_retrieve(id)


Volumes management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_volumes_update

> models::Volume cloud_volumes_update(id, volume)


Volumes management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**volume** | [**Volume**](Volume.md) |  | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

