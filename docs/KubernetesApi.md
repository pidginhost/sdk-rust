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
[**kubernetes_clusters_encryption_create**](KubernetesApi.md#kubernetes_clusters_encryption_create) | **POST** /api/kubernetes/clusters/{id}/encryption/ | 
[**kubernetes_clusters_encryption_recheck_create**](KubernetesApi.md#kubernetes_clusters_encryption_recheck_create) | **POST** /api/kubernetes/clusters/{id}/encryption/recheck/ | 
[**kubernetes_clusters_encryption_reconcile_create**](KubernetesApi.md#kubernetes_clusters_encryption_reconcile_create) | **POST** /api/kubernetes/clusters/{id}/encryption/reconcile/ | 
[**kubernetes_clusters_encryption_retrieve**](KubernetesApi.md#kubernetes_clusters_encryption_retrieve) | **GET** /api/kubernetes/clusters/{id}/encryption/ | 
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
[**kubernetes_clusters_node_operations_cancel_create**](KubernetesApi.md#kubernetes_clusters_node_operations_cancel_create) | **POST** /api/kubernetes/clusters/{cluster_id}/node-operations/{id}/cancel/ | 
[**kubernetes_clusters_node_operations_list**](KubernetesApi.md#kubernetes_clusters_node_operations_list) | **GET** /api/kubernetes/clusters/{cluster_id}/node-operations/ | 
[**kubernetes_clusters_node_operations_resume_create**](KubernetesApi.md#kubernetes_clusters_node_operations_resume_create) | **POST** /api/kubernetes/clusters/{cluster_id}/node-operations/{id}/resume/ | 
[**kubernetes_clusters_node_operations_retrieve**](KubernetesApi.md#kubernetes_clusters_node_operations_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/node-operations/{id}/ | 
[**kubernetes_clusters_node_operations_retry_create**](KubernetesApi.md#kubernetes_clusters_node_operations_retry_create) | **POST** /api/kubernetes/clusters/{cluster_id}/node-operations/{id}/retry/ | 
[**kubernetes_clusters_partial_update**](KubernetesApi.md#kubernetes_clusters_partial_update) | **PATCH** /api/kubernetes/clusters/{id}/ | 
[**kubernetes_clusters_pool_removal_journals_list**](KubernetesApi.md#kubernetes_clusters_pool_removal_journals_list) | **GET** /api/kubernetes/clusters/{cluster_id}/pool-removal-journals/ | 
[**kubernetes_clusters_pool_removal_journals_resume_create**](KubernetesApi.md#kubernetes_clusters_pool_removal_journals_resume_create) | **POST** /api/kubernetes/clusters/{cluster_id}/pool-removal-journals/{id}/resume/ | 
[**kubernetes_clusters_pool_removal_journals_retrieve**](KubernetesApi.md#kubernetes_clusters_pool_removal_journals_retrieve) | **GET** /api/kubernetes/clusters/{cluster_id}/pool-removal-journals/{id}/ | 
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
[**kubernetes_clusters_resource_pools_nodes_reboot_create**](KubernetesApi.md#kubernetes_clusters_resource_pools_nodes_reboot_create) | **POST** /api/kubernetes/clusters/{cluster_id}/resource-pools/{pool_id}/nodes/{id}/reboot/ | 
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
[**kubernetes_clusters_upgrade_lb_create**](KubernetesApi.md#kubernetes_clusters_upgrade_lb_create) | **POST** /api/kubernetes/clusters/{id}/upgrade-lb/ | 



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

> models::ClusterAddResponse kubernetes_clusters_create(cluster_add_request)


Create new k8s cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_add_request** | [**ClusterAddRequest**](ClusterAddRequest.md) |  | [required] |

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


## kubernetes_clusters_encryption_create

> models::ClusterEncryptionOperation kubernetes_clusters_encryption_create(id, cluster_encryption_request)


Enable or disable WireGuard encryption for cluster traffic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**cluster_encryption_request** | [**ClusterEncryptionRequest**](ClusterEncryptionRequest.md) |  | [required] |

### Return type

[**models::ClusterEncryptionOperation**](ClusterEncryptionOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_encryption_recheck_create

> models::ClusterEncryption kubernetes_clusters_encryption_recheck_create(id)


Re-count the workloads that still predate the encryption change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ClusterEncryption**](ClusterEncryption.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_encryption_reconcile_create

> models::ClusterEncryptionOperation kubernetes_clusters_encryption_reconcile_create(id, cluster_encryption_reconcile_request)


Staff only: resolve a cluster whose encryption state is unknown.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**cluster_encryption_reconcile_request** | [**ClusterEncryptionReconcileRequest**](ClusterEncryptionReconcileRequest.md) |  | [required] |

### Return type

[**models::ClusterEncryptionOperation**](ClusterEncryptionOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_encryption_retrieve

> models::ClusterEncryption kubernetes_clusters_encryption_retrieve(id)


Read the cluster's encryption state, restart gate and per-node verification evidence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ClusterEncryption**](ClusterEncryption.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_httproutes_create

> models::HttpRoute kubernetes_clusters_httproutes_create(cluster_id, http_route_request)


Create new HTTPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**http_route_request** | [**HttpRouteRequest**](HttpRouteRequest.md) |  | [required] |

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

> models::HttpRoute kubernetes_clusters_httproutes_partial_update(cluster_id, id, patched_http_route_request)


Partially update HTTPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_http_route_request** | Option<[**PatchedHttpRouteRequest**](PatchedHttpRouteRequest.md)> |  |  |

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

> models::HttpRoute kubernetes_clusters_httproutes_update(cluster_id, id, http_route_request)


Update HTTPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**http_route_request** | [**HttpRouteRequest**](HttpRouteRequest.md) |  | [required] |

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

> models::LbFirewallRule kubernetes_clusters_lb_firewall_create(cluster_id, lb_firewall_rule_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**lb_firewall_rule_request** | Option<[**LbFirewallRuleRequest**](LbFirewallRuleRequest.md)> |  |  |

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

> models::LbFirewallRule kubernetes_clusters_lb_firewall_partial_update(cluster_id, id, patched_lb_firewall_rule_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_lb_firewall_rule_request** | Option<[**PatchedLbFirewallRuleRequest**](PatchedLbFirewallRuleRequest.md)> |  |  |

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

> models::LbFirewallRule kubernetes_clusters_lb_firewall_update(cluster_id, id, lb_firewall_rule_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**lb_firewall_rule_request** | Option<[**LbFirewallRuleRequest**](LbFirewallRuleRequest.md)> |  |  |

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


## kubernetes_clusters_node_operations_cancel_create

> models::NodeOperation kubernetes_clusters_node_operations_cancel_create(cluster_id, id)


Uncordon the node and abort a blocked operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::NodeOperation**](NodeOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_node_operations_list

> models::PaginatedNodeOperationList kubernetes_clusters_node_operations_list(cluster_id, page)


Operation history, status, and the three recovery actions.  Cluster-level rather than node-level on purpose: a successful delete removes the VM row, so an operation addressable only through its node would stop being readable exactly when the customer wants to see how it ended.  None of these routes is gated on `K8S_NODE_OPERATIONS_ENABLED`. Turning new starts off must never strand an operation that is already running -- a cluster with a blocked operation and no way to answer it is a cluster nobody can mutate at all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedNodeOperationList**](PaginatedNodeOperationList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_node_operations_resume_create

> models::NodeOperation kubernetes_clusters_node_operations_resume_create(cluster_id, id)


Staff-only resume of an operation waiting for support.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::NodeOperation**](NodeOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_node_operations_retrieve

> models::NodeOperation kubernetes_clusters_node_operations_retrieve(cluster_id, id)


Operation history, status, and the three recovery actions.  Cluster-level rather than node-level on purpose: a successful delete removes the VM row, so an operation addressable only through its node would stop being readable exactly when the customer wants to see how it ended.  None of these routes is gated on `K8S_NODE_OPERATIONS_ENABLED`. Turning new starts off must never strand an operation that is already running -- a cluster with a blocked operation and no way to answer it is a cluster nobody can mutate at all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::NodeOperation**](NodeOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_node_operations_retry_create

> models::NodeOperation kubernetes_clusters_node_operations_retry_create(cluster_id, id, node_operation_retry_request)


Retry a blocked operation with the overrides that answer its blocker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**node_operation_retry_request** | Option<[**NodeOperationRetryRequest**](NodeOperationRetryRequest.md)> |  |  |

### Return type

[**models::NodeOperation**](NodeOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_partial_update

> models::ClusterDetail kubernetes_clusters_partial_update(id, patched_cluster_detail_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_cluster_detail_request** | Option<[**PatchedClusterDetailRequest**](PatchedClusterDetailRequest.md)> |  |  |

### Return type

[**models::ClusterDetail**](ClusterDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_pool_removal_journals_list

> models::PaginatedPoolRemovalJournalList kubernetes_clusters_pool_removal_journals_list(cluster_id, page)


A downsize or pool deletion, its milestones, and its staff resume.  The list route is not in the spec's table and is here anyway: with retrieve as the only route, a customer whose downsize parked has no way to learn the journal id, and the panel's poll would be the sole path to a published REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedPoolRemovalJournalList**](PaginatedPoolRemovalJournalList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_pool_removal_journals_resume_create

> models::PoolRemovalJournal kubernetes_clusters_pool_removal_journals_resume_create(cluster_id, id)


Staff-only resume of a pool removal waiting for support.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::PoolRemovalJournal**](PoolRemovalJournal.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_pool_removal_journals_retrieve

> models::PoolRemovalJournal kubernetes_clusters_pool_removal_journals_retrieve(cluster_id, id)


A downsize or pool deletion, its milestones, and its staff resume.  The list route is not in the spec's table and is here anyway: with retrieve as the only route, a customer whose downsize parked has no way to learn the journal id, and the panel's poll would be the sole path to a published REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::PoolRemovalJournal**](PoolRemovalJournal.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_port_forwards_create

> models::K8sPortForward kubernetes_clusters_port_forwards_create(cluster_id, k8s_port_forward_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**k8s_port_forward_request** | [**K8sPortForwardRequest**](K8sPortForwardRequest.md) |  | [required] |

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

> models::K8sPortForward kubernetes_clusters_port_forwards_partial_update(cluster_id, id, patched_k8s_port_forward_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_k8s_port_forward_request** | Option<[**PatchedK8sPortForwardRequest**](PatchedK8sPortForwardRequest.md)> |  |  |

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

> models::K8sPortForward kubernetes_clusters_port_forwards_update(cluster_id, id, k8s_port_forward_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**k8s_port_forward_request** | [**K8sPortForwardRequest**](K8sPortForwardRequest.md) |  | [required] |

### Return type

[**models::K8sPortForward**](K8sPortForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_resource_pools_create

> models::ResourcePoolAddResponse kubernetes_clusters_resource_pools_create(cluster_id, resource_pool_add_request)


Create new resource pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**resource_pool_add_request** | [**ResourcePoolAddRequest**](ResourcePoolAddRequest.md) |  | [required] |

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

> models::NodeOperation kubernetes_clusters_resource_pools_nodes_destroy(cluster_id, id, pool_id)


Start a safe delete of one worker node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**pool_id** | **i32** |  | [required] |

### Return type

[**models::NodeOperation**](NodeOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## kubernetes_clusters_resource_pools_nodes_reboot_create

> models::NodeOperation kubernetes_clusters_resource_pools_nodes_reboot_create(cluster_id, id, pool_id, node_operation_reboot_request)


Restart one worker node, draining it first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**pool_id** | **i32** |  | [required] |
**node_operation_reboot_request** | Option<[**NodeOperationRebootRequest**](NodeOperationRebootRequest.md)> |  |  |

### Return type

[**models::NodeOperation**](NodeOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
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

> models::ResourcePool kubernetes_clusters_resource_pools_partial_update(cluster_id, id, patched_resource_pool_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_resource_pool_request** | Option<[**PatchedResourcePoolRequest**](PatchedResourcePoolRequest.md)> |  |  |

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

> models::ResourcePool kubernetes_clusters_resource_pools_update(cluster_id, id, resource_pool_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**resource_pool_request** | Option<[**ResourcePoolRequest**](ResourcePoolRequest.md)> |  |  |

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

> models::TcpRoute kubernetes_clusters_tcproutes_create(cluster_id, tcp_route_request)


Create new TCPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**tcp_route_request** | [**TcpRouteRequest**](TcpRouteRequest.md) |  | [required] |

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

> models::TcpRoute kubernetes_clusters_tcproutes_partial_update(cluster_id, id, patched_tcp_route_request)


Partially update TCPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_tcp_route_request** | Option<[**PatchedTcpRouteRequest**](PatchedTcpRouteRequest.md)> |  |  |

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

> models::TcpRoute kubernetes_clusters_tcproutes_update(cluster_id, id, tcp_route_request)


Update TCPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**tcp_route_request** | [**TcpRouteRequest**](TcpRouteRequest.md) |  | [required] |

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

> models::UdpRoute kubernetes_clusters_udproutes_create(cluster_id, udp_route_request)


Create new UDPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**udp_route_request** | [**UdpRouteRequest**](UdpRouteRequest.md) |  | [required] |

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

> models::UdpRoute kubernetes_clusters_udproutes_partial_update(cluster_id, id, patched_udp_route_request)


Partially update UDPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**patched_udp_route_request** | Option<[**PatchedUdpRouteRequest**](PatchedUdpRouteRequest.md)> |  |  |

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

> models::UdpRoute kubernetes_clusters_udproutes_update(cluster_id, id, udp_route_request)


Update UDPRoute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**udp_route_request** | [**UdpRouteRequest**](UdpRouteRequest.md) |  | [required] |

### Return type

[**models::UdpRoute**](UDPRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_clusters_update

> models::ClusterDetail kubernetes_clusters_update(id, cluster_detail_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**cluster_detail_request** | [**ClusterDetailRequest**](ClusterDetailRequest.md) |  | [required] |

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


## kubernetes_clusters_upgrade_lb_create

> models::LbUpgradePlanResponse kubernetes_clusters_upgrade_lb_create(id, lb_upgrade_request)


Inspect or perform the load-balancer upgrade the server computes for this cluster. The caller never selects a level.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**lb_upgrade_request** | Option<[**LbUpgradeRequest**](LbUpgradeRequest.md)> |  |  |

### Return type

[**models::LbUpgradePlanResponse**](LBUpgradePlanResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

