# \KubernetesApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**kubernetes_cluster_types_list**](KubernetesApi.md#kubernetes_cluster_types_list) | **GET** /api/kubernetes/cluster-types/ | 
[**kubernetes_clusters_connect_vm_create**](KubernetesApi.md#kubernetes_clusters_connect_vm_create) | **POST** /api/kubernetes/clusters/{id}/connect-vm/ | 
[**kubernetes_clusters_connected_vms_retrieve**](KubernetesApi.md#kubernetes_clusters_connected_vms_retrieve) | **GET** /api/kubernetes/clusters/{id}/connected-vms/ | 
[**kubernetes_clusters_create**](KubernetesApi.md#kubernetes_clusters_create) | **POST** /api/kubernetes/clusters/ | 
[**kubernetes_clusters_destroy**](KubernetesApi.md#kubernetes_clusters_destroy) | **DELETE** /api/kubernetes/clusters/{id}/ | 
[**kubernetes_clusters_disconnect_vm_create**](KubernetesApi.md#kubernetes_clusters_disconnect_vm_create) | **POST** /api/kubernetes/clusters/{id}/disconnect-vm/ | 
[**kubernetes_clusters_eligible_vms_retrieve**](KubernetesApi.md#kubernetes_clusters_eligible_vms_retrieve) | **GET** /api/kubernetes/clusters/{id}/eligible-vms/ | 
[**kubernetes_clusters_httproutes_create**](KubernetesApi.md#kubernetes_clusters_httproutes_create) | **POST** /api/kubernetes/clusters/{cluster_id}/httproutes/ | 
[**kubernetes_clusters_httproutes_destroy**](KubernetesApi.md#kubernetes_clusters_httproutes_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/httproutes/{id}/ | 
[**kubernetes_clusters_httproutes_list**](KubernetesApi.md#kubernetes_clusters_httproutes_list) | **GET** /api/kubernetes/clusters/{cluster_id}/httproutes/ | 
[**kubernetes_clusters_httproutes_partial_update**](KubernetesApi.md#kubernetes_clusters_httproutes_partial_update) | **PATCH** /api/kubernetes/clusters/{cluster_id}/httproutes/{id}/ | 
[**kubernetes_clusters_httproutes_retrieve**](KubernetesApi.md#kubernetes_clusters_httproutes_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/httproutes/{id}/ | 
[**kubernetes_clusters_httproutes_update**](KubernetesApi.md#kubernetes_clusters_httproutes_update) | **PUT** /api/kubernetes/clusters/{cluster_id}/httproutes/{id}/ | 
[**kubernetes_clusters_kube_version_upgrade_create**](KubernetesApi.md#kubernetes_clusters_kube_version_upgrade_create) | **POST** /api/kubernetes/clusters/{id}/kube-version-upgrade/ | 
[**kubernetes_clusters_kubeconfig_create**](KubernetesApi.md#kubernetes_clusters_kubeconfig_create) | **POST** /api/kubernetes/clusters/{id}/kubeconfig/ | 
[**kubernetes_clusters_kubeconfig_retrieve**](KubernetesApi.md#kubernetes_clusters_kubeconfig_retrieve) | **GET** /api/kubernetes/clusters/{id}/kubeconfig/ | 
[**kubernetes_clusters_lb_firewall_create**](KubernetesApi.md#kubernetes_clusters_lb_firewall_create) | **POST** /api/kubernetes/clusters/{cluster_id}/lb-firewall/ | 
[**kubernetes_clusters_lb_firewall_destroy**](KubernetesApi.md#kubernetes_clusters_lb_firewall_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/lb-firewall/{id}/ | 
[**kubernetes_clusters_lb_firewall_list**](KubernetesApi.md#kubernetes_clusters_lb_firewall_list) | **GET** /api/kubernetes/clusters/{cluster_id}/lb-firewall/ | 
[**kubernetes_clusters_lb_firewall_partial_update**](KubernetesApi.md#kubernetes_clusters_lb_firewall_partial_update) | **PATCH** /api/kubernetes/clusters/{cluster_id}/lb-firewall/{id}/ | 
[**kubernetes_clusters_lb_firewall_retrieve**](KubernetesApi.md#kubernetes_clusters_lb_firewall_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/lb-firewall/{id}/ | 
[**kubernetes_clusters_lb_firewall_update**](KubernetesApi.md#kubernetes_clusters_lb_firewall_update) | **PUT** /api/kubernetes/clusters/{cluster_id}/lb-firewall/{id}/ | 
[**kubernetes_clusters_list**](KubernetesApi.md#kubernetes_clusters_list) | **GET** /api/kubernetes/clusters/ | 
[**kubernetes_clusters_partial_update**](KubernetesApi.md#kubernetes_clusters_partial_update) | **PATCH** /api/kubernetes/clusters/{id}/ | 
[**kubernetes_clusters_port_forwards_create**](KubernetesApi.md#kubernetes_clusters_port_forwards_create) | **POST** /api/kubernetes/clusters/{cluster_id}/port-forwards/ | 
[**kubernetes_clusters_port_forwards_destroy**](KubernetesApi.md#kubernetes_clusters_port_forwards_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/port-forwards/{id}/ | 
[**kubernetes_clusters_port_forwards_list**](KubernetesApi.md#kubernetes_clusters_port_forwards_list) | **GET** /api/kubernetes/clusters/{cluster_id}/port-forwards/ | 
[**kubernetes_clusters_port_forwards_partial_update**](KubernetesApi.md#kubernetes_clusters_port_forwards_partial_update) | **PATCH** /api/kubernetes/clusters/{cluster_id}/port-forwards/{id}/ | 
[**kubernetes_clusters_port_forwards_retrieve**](KubernetesApi.md#kubernetes_clusters_port_forwards_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/port-forwards/{id}/ | 
[**kubernetes_clusters_port_forwards_update**](KubernetesApi.md#kubernetes_clusters_port_forwards_update) | **PUT** /api/kubernetes/clusters/{cluster_id}/port-forwards/{id}/ | 
[**kubernetes_clusters_resource_pools_create**](KubernetesApi.md#kubernetes_clusters_resource_pools_create) | **POST** /api/kubernetes/clusters/{cluster_id}/resource-pools/ | 
[**kubernetes_clusters_resource_pools_destroy**](KubernetesApi.md#kubernetes_clusters_resource_pools_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/resource-pools/{id}/ | 
[**kubernetes_clusters_resource_pools_list**](KubernetesApi.md#kubernetes_clusters_resource_pools_list) | **GET** /api/kubernetes/clusters/{cluster_id}/resource-pools/ | 
[**kubernetes_clusters_resource_pools_nodes_destroy**](KubernetesApi.md#kubernetes_clusters_resource_pools_nodes_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/resource-pools/{pool_id}/nodes/{id}/ | 
[**kubernetes_clusters_resource_pools_nodes_list**](KubernetesApi.md#kubernetes_clusters_resource_pools_nodes_list) | **GET** /api/kubernetes/clusters/{cluster_id}/resource-pools/{pool_id}/nodes/ | 
[**kubernetes_clusters_resource_pools_nodes_metrics_retrieve**](KubernetesApi.md#kubernetes_clusters_resource_pools_nodes_metrics_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/resource-pools/{pool_id}/nodes/{id}/metrics/ | 
[**kubernetes_clusters_resource_pools_nodes_retrieve**](KubernetesApi.md#kubernetes_clusters_resource_pools_nodes_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/resource-pools/{pool_id}/nodes/{id}/ | 
[**kubernetes_clusters_resource_pools_nodes_rrd_retrieve**](KubernetesApi.md#kubernetes_clusters_resource_pools_nodes_rrd_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/resource-pools/{pool_id}/nodes/{id}/rrd/ | 
[**kubernetes_clusters_resource_pools_partial_update**](KubernetesApi.md#kubernetes_clusters_resource_pools_partial_update) | **PATCH** /api/kubernetes/clusters/{cluster_id}/resource-pools/{id}/ | 
[**kubernetes_clusters_resource_pools_retrieve**](KubernetesApi.md#kubernetes_clusters_resource_pools_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/resource-pools/{id}/ | 
[**kubernetes_clusters_resource_pools_update**](KubernetesApi.md#kubernetes_clusters_resource_pools_update) | **PUT** /api/kubernetes/clusters/{cluster_id}/resource-pools/{id}/ | 
[**kubernetes_clusters_retrieve**](KubernetesApi.md#kubernetes_clusters_retrieve) | **GET** /api/kubernetes/clusters/{id}/ | 
[**kubernetes_clusters_talos_version_upgrade_create**](KubernetesApi.md#kubernetes_clusters_talos_version_upgrade_create) | **POST** /api/kubernetes/clusters/{id}/talos-version-upgrade/ | 
[**kubernetes_clusters_tcproutes_create**](KubernetesApi.md#kubernetes_clusters_tcproutes_create) | **POST** /api/kubernetes/clusters/{cluster_id}/tcproutes/ | 
[**kubernetes_clusters_tcproutes_destroy**](KubernetesApi.md#kubernetes_clusters_tcproutes_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/tcproutes/{id}/ | 
[**kubernetes_clusters_tcproutes_list**](KubernetesApi.md#kubernetes_clusters_tcproutes_list) | **GET** /api/kubernetes/clusters/{cluster_id}/tcproutes/ | 
[**kubernetes_clusters_tcproutes_partial_update**](KubernetesApi.md#kubernetes_clusters_tcproutes_partial_update) | **PATCH** /api/kubernetes/clusters/{cluster_id}/tcproutes/{id}/ | 
[**kubernetes_clusters_tcproutes_retrieve**](KubernetesApi.md#kubernetes_clusters_tcproutes_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/tcproutes/{id}/ | 
[**kubernetes_clusters_tcproutes_update**](KubernetesApi.md#kubernetes_clusters_tcproutes_update) | **PUT** /api/kubernetes/clusters/{cluster_id}/tcproutes/{id}/ | 
[**kubernetes_clusters_toggle_cloud_vm_access_create**](KubernetesApi.md#kubernetes_clusters_toggle_cloud_vm_access_create) | **POST** /api/kubernetes/clusters/{id}/toggle-cloud-vm-access/ | 
[**kubernetes_clusters_udproutes_create**](KubernetesApi.md#kubernetes_clusters_udproutes_create) | **POST** /api/kubernetes/clusters/{cluster_id}/udproutes/ | 
[**kubernetes_clusters_udproutes_destroy**](KubernetesApi.md#kubernetes_clusters_udproutes_destroy) | **DELETE** /api/kubernetes/clusters/{cluster_id}/udproutes/{id}/ | 
[**kubernetes_clusters_udproutes_list**](KubernetesApi.md#kubernetes_clusters_udproutes_list) | **GET** /api/kubernetes/clusters/{cluster_id}/udproutes/ | 
[**kubernetes_clusters_udproutes_partial_update**](KubernetesApi.md#kubernetes_clusters_udproutes_partial_update) | **PATCH** /api/kubernetes/clusters/{cluster_id}/udproutes/{id}/ | 
[**kubernetes_clusters_udproutes_retrieve**](KubernetesApi.md#kubernetes_clusters_udproutes_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/udproutes/{id}/ | 
[**kubernetes_clusters_udproutes_update**](KubernetesApi.md#kubernetes_clusters_udproutes_update) | **PUT** /api/kubernetes/clusters/{cluster_id}/udproutes/{id}/ | 
[**kubernetes_clusters_update**](KubernetesApi.md#kubernetes_clusters_update) | **PUT** /api/kubernetes/clusters/{id}/ | 
[**kubernetes_clusters_upgrade_feature_create**](KubernetesApi.md#kubernetes_clusters_upgrade_feature_create) | **POST** /api/kubernetes/clusters/{id}/upgrade-feature/ | 



## kubernetes_cluster_types_list

> models::PaginatedClusterTypeList kubernetes_cluster_types_list(page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedClusterTypeList**](PaginatedClusterTypeList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_connect_vm_create

> models::ConnectVmResponse kubernetes_clusters_connect_vm_create(id, connect_vm_request)


Connect a cloud VM to the cluster private network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**connect_vm_request** | [**ConnectVmRequest**](ConnectVmRequest.md) |  | [required] |

### Return type

[**models::ConnectVmResponse**](ConnectVMResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_connected_vms_retrieve

> models::ConnectedVmsResponse kubernetes_clusters_connected_vms_retrieve(id)


List cloud VMs connected to the cluster private network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ConnectedVmsResponse**](ConnectedVMsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_create

> models::ClusterAddResponse kubernetes_clusters_create(cluster_add)


Create new k8s cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_add** | [**ClusterAdd**](ClusterAdd.md) |  | [required] |

### Return type

[**models::ClusterAddResponse**](ClusterAddResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_destroy

> kubernetes_clusters_destroy(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_disconnect_vm_create

> models::DisconnectVmResponse kubernetes_clusters_disconnect_vm_create(id, disconnect_vm_request)


Disconnect a cloud VM from the cluster private network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**disconnect_vm_request** | [**DisconnectVmRequest**](DisconnectVmRequest.md) |  | [required] |

### Return type

[**models::DisconnectVmResponse**](DisconnectVMResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_eligible_vms_retrieve

> models::EligibleVmsResponse kubernetes_clusters_eligible_vms_retrieve(id)


List cloud VMs eligible for connection to this cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::EligibleVmsResponse**](EligibleVMsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_create

> models::HttpRoute kubernetes_clusters_httproutes_create(cluster_id, http_route)


Create new HTTPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**http_route** | [**HttpRoute**](HttpRoute.md) |  | [required] |

### Return type

[**models::HttpRoute**](HTTPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_destroy

> kubernetes_clusters_httproutes_destroy(cluster_id, id)


ViewSet for managing HTTPRoute resources.  HTTPRoutes expose HTTP/HTTPS services through the Gateway with optional automatic TLS certificate issuance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_list

> models::PaginatedHttpRouteList kubernetes_clusters_httproutes_list(cluster_id, page)


ViewSet for managing HTTPRoute resources.  HTTPRoutes expose HTTP/HTTPS services through the Gateway with optional automatic TLS certificate issuance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedHttpRouteList**](PaginatedHTTPRouteList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_partial_update

> models::HttpRoute kubernetes_clusters_httproutes_partial_update(cluster_id, id, patched_http_route)


Partially update HTTPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_http_route** | Option<[**PatchedHttpRoute**](PatchedHttpRoute.md)> |  |  |

### Return type

[**models::HttpRoute**](HTTPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_retrieve

> models::HttpRoute kubernetes_clusters_httproutes_retrieve(cluster_id, id)


ViewSet for managing HTTPRoute resources.  HTTPRoutes expose HTTP/HTTPS services through the Gateway with optional automatic TLS certificate issuance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::HttpRoute**](HTTPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_update

> models::HttpRoute kubernetes_clusters_httproutes_update(cluster_id, id, http_route)


Update HTTPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**http_route** | [**HttpRoute**](HttpRoute.md) |  | [required] |

### Return type

[**models::HttpRoute**](HTTPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_kube_version_upgrade_create

> models::KubeUpgradeResponse kubernetes_clusters_kube_version_upgrade_create(id)


Upgrade kubernetes to the next available version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::KubeUpgradeResponse**](KubeUpgradeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_kubeconfig_create

> String kubernetes_clusters_kubeconfig_create(id)


Download kubeconfig file. Use POST to generate a new kubeconfig.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_kubeconfig_retrieve

> String kubernetes_clusters_kubeconfig_retrieve(id)


Download kubeconfig file. Use POST to generate a new kubeconfig.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_lb_firewall_create

> models::LbFirewallRule kubernetes_clusters_lb_firewall_create(cluster_id, lb_firewall_rule)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**lb_firewall_rule** | Option<[**LbFirewallRule**](LbFirewallRule.md)> |  |  |

### Return type

[**models::LbFirewallRule**](LBFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_lb_firewall_destroy

> kubernetes_clusters_lb_firewall_destroy(cluster_id, id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_lb_firewall_list

> models::PaginatedLbFirewallRuleList kubernetes_clusters_lb_firewall_list(cluster_id, page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedLbFirewallRuleList**](PaginatedLBFirewallRuleList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_lb_firewall_partial_update

> models::LbFirewallRule kubernetes_clusters_lb_firewall_partial_update(cluster_id, id, patched_lb_firewall_rule)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_lb_firewall_rule** | Option<[**PatchedLbFirewallRule**](PatchedLbFirewallRule.md)> |  |  |

### Return type

[**models::LbFirewallRule**](LBFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_lb_firewall_retrieve

> models::LbFirewallRule kubernetes_clusters_lb_firewall_retrieve(cluster_id, id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::LbFirewallRule**](LBFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_lb_firewall_update

> models::LbFirewallRule kubernetes_clusters_lb_firewall_update(cluster_id, id, lb_firewall_rule)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**lb_firewall_rule** | Option<[**LbFirewallRule**](LbFirewallRule.md)> |  |  |

### Return type

[**models::LbFirewallRule**](LBFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_list

> models::PaginatedClusterDetailList kubernetes_clusters_list(page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedClusterDetailList**](PaginatedClusterDetailList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_partial_update

> models::ClusterDetail kubernetes_clusters_partial_update(id, patched_cluster_detail)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_cluster_detail** | Option<[**PatchedClusterDetail**](PatchedClusterDetail.md)> |  |  |

### Return type

[**models::ClusterDetail**](ClusterDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_create

> models::K8sPortForward kubernetes_clusters_port_forwards_create(cluster_id, k8s_port_forward)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**k8s_port_forward** | [**K8sPortForward**](K8sPortForward.md) |  | [required] |

### Return type

[**models::K8sPortForward**](K8sPortForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_destroy

> kubernetes_clusters_port_forwards_destroy(cluster_id, id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_list

> models::PaginatedK8sPortForwardList kubernetes_clusters_port_forwards_list(cluster_id, page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedK8sPortForwardList**](PaginatedK8sPortForwardList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_partial_update

> models::K8sPortForward kubernetes_clusters_port_forwards_partial_update(cluster_id, id, patched_k8s_port_forward)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_k8s_port_forward** | Option<[**PatchedK8sPortForward**](PatchedK8sPortForward.md)> |  |  |

### Return type

[**models::K8sPortForward**](K8sPortForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_retrieve

> models::K8sPortForward kubernetes_clusters_port_forwards_retrieve(cluster_id, id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::K8sPortForward**](K8sPortForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_update

> models::K8sPortForward kubernetes_clusters_port_forwards_update(cluster_id, id, k8s_port_forward)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**k8s_port_forward** | [**K8sPortForward**](K8sPortForward.md) |  | [required] |

### Return type

[**models::K8sPortForward**](K8sPortForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_create

> models::ResourcePoolAddResponse kubernetes_clusters_resource_pools_create(cluster_id, resource_pool_add)


Create new resource pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**resource_pool_add** | [**ResourcePoolAdd**](ResourcePoolAdd.md) |  | [required] |

### Return type

[**models::ResourcePoolAddResponse**](ResourcePoolAddResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_destroy

> kubernetes_clusters_resource_pools_destroy(cluster_id, id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_list

> models::PaginatedResourcePoolList kubernetes_clusters_resource_pools_list(cluster_id, page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedResourcePoolList**](PaginatedResourcePoolList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_nodes_destroy

> kubernetes_clusters_resource_pools_nodes_destroy(cluster_id, id, pool_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**pool_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_nodes_list

> models::PaginatedResourcePoolNodeList kubernetes_clusters_resource_pools_nodes_list(cluster_id, pool_id, page)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**pool_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedResourcePoolNodeList**](PaginatedResourcePoolNodeList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_nodes_metrics_retrieve

> models::NodeMetricsResponse kubernetes_clusters_resource_pools_nodes_metrics_retrieve(cluster_id, id, pool_id)


Get real-time metrics for a node VM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**pool_id** | **i32** |  | [required] |

### Return type

[**models::NodeMetricsResponse**](NodeMetricsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_nodes_retrieve

> models::ResourcePoolNode kubernetes_clusters_resource_pools_nodes_retrieve(cluster_id, id, pool_id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**pool_id** | **i32** |  | [required] |

### Return type

[**models::ResourcePoolNode**](ResourcePoolNode.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_nodes_rrd_retrieve

> models::NodeRrdResponse kubernetes_clusters_resource_pools_nodes_rrd_retrieve(cluster_id, id, pool_id, timeframe)


Get RRD (historical) metrics data for a node VM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**pool_id** | **i32** |  | [required] |
**timeframe** | Option<**String**> | Window of recorded data to return. |  |[default to hour]

### Return type

[**models::NodeRrdResponse**](NodeRRDResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_partial_update

> models::ResourcePool kubernetes_clusters_resource_pools_partial_update(cluster_id, id, patched_resource_pool)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_resource_pool** | Option<[**PatchedResourcePool**](PatchedResourcePool.md)> |  |  |

### Return type

[**models::ResourcePool**](ResourcePool.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_retrieve

> models::ResourcePool kubernetes_clusters_resource_pools_retrieve(cluster_id, id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ResourcePool**](ResourcePool.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_update

> models::ResourcePool kubernetes_clusters_resource_pools_update(cluster_id, id, resource_pool)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**resource_pool** | Option<[**ResourcePool**](ResourcePool.md)> |  |  |

### Return type

[**models::ResourcePool**](ResourcePool.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_retrieve

> models::ClusterDetail kubernetes_clusters_retrieve(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ClusterDetail**](ClusterDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_talos_version_upgrade_create

> models::TalosUpgradeResponse kubernetes_clusters_talos_version_upgrade_create(id)


Upgrade Talos to the next available version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TalosUpgradeResponse**](TalosUpgradeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_tcproutes_create

> models::TcpRoute kubernetes_clusters_tcproutes_create(cluster_id, tcp_route)


Create new TCPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**tcp_route** | [**TcpRoute**](TcpRoute.md) |  | [required] |

### Return type

[**models::TcpRoute**](TCPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_tcproutes_destroy

> kubernetes_clusters_tcproutes_destroy(cluster_id, id)


ViewSet for managing TCPRoute resources.  TCPRoutes expose TCP services through the Gateway on specific external ports. Reserved ports (22, 6443, 50000, 50001) cannot be exposed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_tcproutes_list

> models::PaginatedTcpRouteList kubernetes_clusters_tcproutes_list(cluster_id, page)


ViewSet for managing TCPRoute resources.  TCPRoutes expose TCP services through the Gateway on specific external ports. Reserved ports (22, 6443, 50000, 50001) cannot be exposed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedTcpRouteList**](PaginatedTCPRouteList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_tcproutes_partial_update

> models::TcpRoute kubernetes_clusters_tcproutes_partial_update(cluster_id, id, patched_tcp_route)


Partially update TCPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_tcp_route** | Option<[**PatchedTcpRoute**](PatchedTcpRoute.md)> |  |  |

### Return type

[**models::TcpRoute**](TCPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_tcproutes_retrieve

> models::TcpRoute kubernetes_clusters_tcproutes_retrieve(cluster_id, id)


ViewSet for managing TCPRoute resources.  TCPRoutes expose TCP services through the Gateway on specific external ports. Reserved ports (22, 6443, 50000, 50001) cannot be exposed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::TcpRoute**](TCPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_tcproutes_update

> models::TcpRoute kubernetes_clusters_tcproutes_update(cluster_id, id, tcp_route)


Update TCPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**tcp_route** | [**TcpRoute**](TcpRoute.md) |  | [required] |

### Return type

[**models::TcpRoute**](TCPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_toggle_cloud_vm_access_create

> models::ToggleCloudVmAccessResponse kubernetes_clusters_toggle_cloud_vm_access_create(id)


Toggle cloud VM access for this cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToggleCloudVmAccessResponse**](ToggleCloudVMAccessResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_udproutes_create

> models::UdpRoute kubernetes_clusters_udproutes_create(cluster_id, udp_route)


Create new UDPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**udp_route** | [**UdpRoute**](UdpRoute.md) |  | [required] |

### Return type

[**models::UdpRoute**](UDPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_udproutes_destroy

> kubernetes_clusters_udproutes_destroy(cluster_id, id)


ViewSet for managing UDPRoute resources.  UDPRoutes expose UDP services through the Gateway on specific external ports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_udproutes_list

> models::PaginatedUdpRouteList kubernetes_clusters_udproutes_list(cluster_id, page)


ViewSet for managing UDPRoute resources.  UDPRoutes expose UDP services through the Gateway on specific external ports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedUdpRouteList**](PaginatedUDPRouteList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_udproutes_partial_update

> models::UdpRoute kubernetes_clusters_udproutes_partial_update(cluster_id, id, patched_udp_route)


Partially update UDPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_udp_route** | Option<[**PatchedUdpRoute**](PatchedUdpRoute.md)> |  |  |

### Return type

[**models::UdpRoute**](UDPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_udproutes_retrieve

> models::UdpRoute kubernetes_clusters_udproutes_retrieve(cluster_id, id)


ViewSet for managing UDPRoute resources.  UDPRoutes expose UDP services through the Gateway on specific external ports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::UdpRoute**](UDPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_udproutes_update

> models::UdpRoute kubernetes_clusters_udproutes_update(cluster_id, id, udp_route)


Update UDPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**udp_route** | [**UdpRoute**](UdpRoute.md) |  | [required] |

### Return type

[**models::UdpRoute**](UDPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_update

> models::ClusterDetail kubernetes_clusters_update(id, cluster_detail)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**cluster_detail** | [**ClusterDetail**](ClusterDetail.md) |  | [required] |

### Return type

[**models::ClusterDetail**](ClusterDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_upgrade_feature_create

> models::FeatureUpgradeResponse kubernetes_clusters_upgrade_feature_create(id, feature_upgrade_request)


Upgrade a cluster feature to the latest compatible version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**feature_upgrade_request** | [**FeatureUpgradeRequest**](FeatureUpgradeRequest.md) |  | [required] |

### Return type

[**models::FeatureUpgradeResponse**](FeatureUpgradeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

