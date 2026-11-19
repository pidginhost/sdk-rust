# \CloudApi

All URIs are relative to *https://www.pidginhost.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_buckets_create**](CloudApi.md#cloud_buckets_create) | **POST** /api/cloud/buckets/ | 
[**cloud_buckets_credentials_reveal_create**](CloudApi.md#cloud_buckets_credentials_reveal_create) | **POST** /api/cloud/buckets/{id}/credentials/reveal/ | 
[**cloud_buckets_credentials_rotate_create**](CloudApi.md#cloud_buckets_credentials_rotate_create) | **POST** /api/cloud/buckets/{id}/credentials/rotate/ | 
[**cloud_buckets_destroy**](CloudApi.md#cloud_buckets_destroy) | **DELETE** /api/cloud/buckets/{id}/ | 
[**cloud_buckets_destroy_protection_create**](CloudApi.md#cloud_buckets_destroy_protection_create) | **POST** /api/cloud/buckets/{id}/destroy-protection/ | 
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
[**cloud_gateways_create**](CloudApi.md#cloud_gateways_create) | **POST** /api/cloud/gateways/ | 
[**cloud_gateways_destroy**](CloudApi.md#cloud_gateways_destroy) | **DELETE** /api/cloud/gateways/{id}/ | 
[**cloud_gateways_firewall_rules_create**](CloudApi.md#cloud_gateways_firewall_rules_create) | **POST** /api/cloud/gateways/{gateway_id}/firewall-rules/ | 
[**cloud_gateways_firewall_rules_destroy**](CloudApi.md#cloud_gateways_firewall_rules_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/firewall-rules/{id}/ | 
[**cloud_gateways_firewall_rules_list**](CloudApi.md#cloud_gateways_firewall_rules_list) | **GET** /api/cloud/gateways/{gateway_id}/firewall-rules/ | 
[**cloud_gateways_firewall_rules_move_create**](CloudApi.md#cloud_gateways_firewall_rules_move_create) | **POST** /api/cloud/gateways/{gateway_id}/firewall-rules/{id}/move/ | 
[**cloud_gateways_firewall_rules_partial_update**](CloudApi.md#cloud_gateways_firewall_rules_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/firewall-rules/{id}/ | 
[**cloud_gateways_firewall_rules_retrieve**](CloudApi.md#cloud_gateways_firewall_rules_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/firewall-rules/{id}/ | 
[**cloud_gateways_list**](CloudApi.md#cloud_gateways_list) | **GET** /api/cloud/gateways/ | 
[**cloud_gateways_listeners_create**](CloudApi.md#cloud_gateways_listeners_create) | **POST** /api/cloud/gateways/{gateway_id}/listeners/ | 
[**cloud_gateways_listeners_destroy**](CloudApi.md#cloud_gateways_listeners_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/listeners/{id}/ | 
[**cloud_gateways_listeners_list**](CloudApi.md#cloud_gateways_listeners_list) | **GET** /api/cloud/gateways/{gateway_id}/listeners/ | 
[**cloud_gateways_listeners_partial_update**](CloudApi.md#cloud_gateways_listeners_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/listeners/{id}/ | 
[**cloud_gateways_listeners_retrieve**](CloudApi.md#cloud_gateways_listeners_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/listeners/{id}/ | 
[**cloud_gateways_listeners_routes_create**](CloudApi.md#cloud_gateways_listeners_routes_create) | **POST** /api/cloud/gateways/{gateway_id}/listeners/{listener_id}/routes/ | 
[**cloud_gateways_listeners_routes_destroy**](CloudApi.md#cloud_gateways_listeners_routes_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/listeners/{listener_id}/routes/{id}/ | 
[**cloud_gateways_listeners_routes_list**](CloudApi.md#cloud_gateways_listeners_routes_list) | **GET** /api/cloud/gateways/{gateway_id}/listeners/{listener_id}/routes/ | 
[**cloud_gateways_listeners_routes_partial_update**](CloudApi.md#cloud_gateways_listeners_routes_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/listeners/{listener_id}/routes/{id}/ | 
[**cloud_gateways_listeners_routes_retrieve**](CloudApi.md#cloud_gateways_listeners_routes_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/listeners/{listener_id}/routes/{id}/ | 
[**cloud_gateways_networks_create**](CloudApi.md#cloud_gateways_networks_create) | **POST** /api/cloud/gateways/{gateway_id}/networks/ | 
[**cloud_gateways_networks_destroy**](CloudApi.md#cloud_gateways_networks_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/networks/{network_id}/ | 
[**cloud_gateways_networks_list**](CloudApi.md#cloud_gateways_networks_list) | **GET** /api/cloud/gateways/{gateway_id}/networks/ | 
[**cloud_gateways_networks_partial_update**](CloudApi.md#cloud_gateways_networks_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/networks/{network_id}/ | 
[**cloud_gateways_networks_retrieve**](CloudApi.md#cloud_gateways_networks_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/networks/{network_id}/ | 
[**cloud_gateways_operations_list**](CloudApi.md#cloud_gateways_operations_list) | **GET** /api/cloud/gateways/{gateway_id}/operations/ | 
[**cloud_gateways_operations_retrieve**](CloudApi.md#cloud_gateways_operations_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/operations/{id}/ | 
[**cloud_gateways_partial_update**](CloudApi.md#cloud_gateways_partial_update) | **PATCH** /api/cloud/gateways/{id}/ | 
[**cloud_gateways_pools_backends_create**](CloudApi.md#cloud_gateways_pools_backends_create) | **POST** /api/cloud/gateways/{gateway_id}/pools/{pool_id}/backends/ | 
[**cloud_gateways_pools_backends_destroy**](CloudApi.md#cloud_gateways_pools_backends_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/pools/{pool_id}/backends/{id}/ | 
[**cloud_gateways_pools_backends_list**](CloudApi.md#cloud_gateways_pools_backends_list) | **GET** /api/cloud/gateways/{gateway_id}/pools/{pool_id}/backends/ | 
[**cloud_gateways_pools_backends_partial_update**](CloudApi.md#cloud_gateways_pools_backends_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/pools/{pool_id}/backends/{id}/ | 
[**cloud_gateways_pools_backends_retrieve**](CloudApi.md#cloud_gateways_pools_backends_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/pools/{pool_id}/backends/{id}/ | 
[**cloud_gateways_pools_create**](CloudApi.md#cloud_gateways_pools_create) | **POST** /api/cloud/gateways/{gateway_id}/pools/ | 
[**cloud_gateways_pools_destroy**](CloudApi.md#cloud_gateways_pools_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/pools/{id}/ | 
[**cloud_gateways_pools_list**](CloudApi.md#cloud_gateways_pools_list) | **GET** /api/cloud/gateways/{gateway_id}/pools/ | 
[**cloud_gateways_pools_partial_update**](CloudApi.md#cloud_gateways_pools_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/pools/{id}/ | 
[**cloud_gateways_pools_retrieve**](CloudApi.md#cloud_gateways_pools_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/pools/{id}/ | 
[**cloud_gateways_port_forwards_create**](CloudApi.md#cloud_gateways_port_forwards_create) | **POST** /api/cloud/gateways/{gateway_id}/port-forwards/ | 
[**cloud_gateways_port_forwards_destroy**](CloudApi.md#cloud_gateways_port_forwards_destroy) | **DELETE** /api/cloud/gateways/{gateway_id}/port-forwards/{id}/ | 
[**cloud_gateways_port_forwards_list**](CloudApi.md#cloud_gateways_port_forwards_list) | **GET** /api/cloud/gateways/{gateway_id}/port-forwards/ | 
[**cloud_gateways_port_forwards_partial_update**](CloudApi.md#cloud_gateways_port_forwards_partial_update) | **PATCH** /api/cloud/gateways/{gateway_id}/port-forwards/{id}/ | 
[**cloud_gateways_port_forwards_retrieve**](CloudApi.md#cloud_gateways_port_forwards_retrieve) | **GET** /api/cloud/gateways/{gateway_id}/port-forwards/{id}/ | 
[**cloud_gateways_reenroll_create**](CloudApi.md#cloud_gateways_reenroll_create) | **POST** /api/cloud/gateways/{id}/reenroll/ | 
[**cloud_gateways_retrieve**](CloudApi.md#cloud_gateways_retrieve) | **GET** /api/cloud/gateways/{id}/ | 
[**cloud_gateways_status_retrieve**](CloudApi.md#cloud_gateways_status_retrieve) | **GET** /api/cloud/gateways/{id}/status/ | 
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
[**cloud_private_networks_apply_route_create**](CloudApi.md#cloud_private_networks_apply_route_create) | **POST** /api/cloud/private-networks/{id}/apply-route/ | 
[**cloud_private_networks_create**](CloudApi.md#cloud_private_networks_create) | **POST** /api/cloud/private-networks/ | 
[**cloud_private_networks_destroy**](CloudApi.md#cloud_private_networks_destroy) | **DELETE** /api/cloud/private-networks/{id}/ | 
[**cloud_private_networks_list**](CloudApi.md#cloud_private_networks_list) | **GET** /api/cloud/private-networks/ | 
[**cloud_private_networks_partial_update**](CloudApi.md#cloud_private_networks_partial_update) | **PATCH** /api/cloud/private-networks/{id}/ | 
[**cloud_private_networks_remove_server_create**](CloudApi.md#cloud_private_networks_remove_server_create) | **POST** /api/cloud/private-networks/{id}/remove-server/ | 
[**cloud_private_networks_retrieve**](CloudApi.md#cloud_private_networks_retrieve) | **GET** /api/cloud/private-networks/{id}/ | 
[**cloud_private_networks_route_status_retrieve**](CloudApi.md#cloud_private_networks_route_status_retrieve) | **GET** /api/cloud/private-networks/{id}/route-status/ | 
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
[**cloud_servers_monitoring_retrieve**](CloudApi.md#cloud_servers_monitoring_retrieve) | **GET** /api/cloud/servers/{id}/monitoring/ | 
[**cloud_servers_monitoring_update**](CloudApi.md#cloud_servers_monitoring_update) | **PUT** /api/cloud/servers/{id}/monitoring/ | 
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
[**cloud_servers_traffic_retrieve**](CloudApi.md#cloud_servers_traffic_retrieve) | **GET** /api/cloud/servers/{id}/traffic/ | 
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

> models::Bucket cloud_buckets_create(bucket_create_request)


Create a bucket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_create_request** | [**BucketCreateRequest**](BucketCreateRequest.md) |  | [required] |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_credentials_reveal_create

> models::BucketCredentials cloud_buckets_credentials_reveal_create(id)


Reveal bucket credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |

### Return type

[**models::BucketCredentials**](BucketCredentials.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_credentials_rotate_create

> models::BucketCredentials cloud_buckets_credentials_rotate_create(id)


Rotate bucket credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |

### Return type

[**models::BucketCredentials**](BucketCredentials.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_destroy

> models::BucketCancelResponse cloud_buckets_destroy(id)


Cancel a bucket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |

### Return type

[**models::BucketCancelResponse**](BucketCancelResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_buckets_destroy_protection_create

> models::DestroyProtection cloud_buckets_destroy_protection_create(id, destroy_protection_request)


Enable or disable destroy protection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**destroy_protection_request** | [**DestroyProtectionRequest**](DestroyProtectionRequest.md) |  | [required] |

### Return type

[**models::DestroyProtection**](DestroyProtection.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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

> models::Bucket cloud_buckets_resize_create(id, bucket_resize_request)


Resize a bucket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**bucket_resize_request** | [**BucketResizeRequest**](BucketResizeRequest.md) |  | [required] |

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

> models::Bucket cloud_buckets_visibility_create(id, bucket_visibility_request)


Set bucket visibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this S3 bucket. | [required] |
**bucket_visibility_request** | [**BucketVisibilityRequest**](BucketVisibilityRequest.md) |  | [required] |

### Return type

[**models::Bucket**](Bucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_create

> models::FirewallRulesSet cloud_firewall_rules_set_create(firewall_rules_set_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_rules_set_request** | [**FirewallRulesSetRequest**](FirewallRulesSetRequest.md) |  | [required] |

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

> models::FirewallRulesSet cloud_firewall_rules_set_partial_update(id, patched_firewall_rules_set_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this firewall rules set. | [required] |
**patched_firewall_rules_set_request** | Option<[**PatchedFirewallRulesSetRequest**](PatchedFirewallRulesSetRequest.md)> |  |  |

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

> models::FirewallRule cloud_firewall_rules_set_rules_create(rules_set_id, firewall_rule_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rules_set_id** | **String** |  | [required] |
**firewall_rule_request** | [**FirewallRuleRequest**](FirewallRuleRequest.md) |  | [required] |

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

> models::FirewallRule cloud_firewall_rules_set_rules_partial_update(rule_id, rules_set_id, patched_firewall_rule_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**rules_set_id** | **String** |  | [required] |
**patched_firewall_rule_request** | Option<[**PatchedFirewallRuleRequest**](PatchedFirewallRuleRequest.md)> |  |  |

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

> models::FirewallRule cloud_firewall_rules_set_rules_update(rule_id, rules_set_id, firewall_rule_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**rules_set_id** | **String** |  | [required] |
**firewall_rule_request** | [**FirewallRuleRequest**](FirewallRuleRequest.md) |  | [required] |

### Return type

[**models::FirewallRule**](FirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_firewall_rules_set_update

> models::FirewallRulesSet cloud_firewall_rules_set_update(id, firewall_rules_set_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this firewall rules set. | [required] |
**firewall_rules_set_request** | [**FirewallRulesSetRequest**](FirewallRulesSetRequest.md) |  | [required] |

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

> models::FloatingIpv4 cloud_floating_ipv4_create(floating_ipv4_create_request)


Manage floating IPv4 addresses. A floating IP can be authorized on multiple VMs simultaneously; the customer asserts ownership inside the guest via keepalived/VRRP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ipv4_create_request** | Option<[**FloatingIpv4CreateRequest**](FloatingIpv4CreateRequest.md)> |  |  |

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

> models::ReverseDns cloud_floating_ipv4_rdns_create(id, reverse_dns_request)


Get or update reverse DNS (PTR) for the IPv4 address wrapped by this floating IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv4. | [required] |
**reverse_dns_request** | [**ReverseDnsRequest**](ReverseDnsRequest.md) |  | [required] |

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

> models::FloatingIpv6 cloud_floating_ipv6_create(floating_ipv6_create_request)


Manage floating IPv6 addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ipv6_create_request** | Option<[**FloatingIpv6CreateRequest**](FloatingIpv6CreateRequest.md)> |  |  |

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

> models::ReverseDns cloud_floating_ipv6_rdns_create(id, reverse_dns_request)


Get or update reverse DNS (PTR) for the IPv6 address wrapped by this floating IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this floating IPv6. | [required] |
**reverse_dns_request** | [**ReverseDnsRequest**](ReverseDnsRequest.md) |  | [required] |

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


## cloud_gateways_create

> models::Gateway cloud_gateways_create(gateway_create_request)


Reserve a gateway and queue its provisioning (spec 11.1); its state shows the progress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_create_request** | [**GatewayCreateRequest**](GatewayCreateRequest.md) |  | [required] |

### Return type

[**models::Gateway**](Gateway.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_destroy

> models::GatewayOperation cloud_gateways_destroy(id, networks, revision, servers)


Delete the gateway (spec 11.5): its VM is destroyed, then its networks and IPv4 are released. Without the preview's revision, networks and servers, 409 with a fresh preview.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**networks** | Option<[**Vec<i32>**](I32.md)> | The preview's network ids. |  |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |
**servers** | Option<[**Vec<i32>**](I32.md)> | The preview's server ids. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_firewall_rules_create

> models::GatewayFirewallRule cloud_gateways_firewall_rules_create(gateway_id, gateway_firewall_rule_write_request)


Add a rule after the last one (spec 5.7); it is pending until the agent applies its revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**gateway_firewall_rule_write_request** | [**GatewayFirewallRuleWriteRequest**](GatewayFirewallRuleWriteRequest.md) |  | [required] |

### Return type

[**models::GatewayFirewallRule**](GatewayFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_firewall_rules_destroy

> models::GatewayOperation cloud_gateways_firewall_rules_destroy(gateway_id, id, revision)


Delete a rule (spec 5.11): 202 with the record the agent's acknowledgement completes. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_firewall_rules_list

> models::PaginatedGatewayFirewallRuleList cloud_gateways_firewall_rules_list(gateway_id, page)


A gateway's firewall rules, in evaluation order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayFirewallRuleList**](PaginatedGatewayFirewallRuleList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_firewall_rules_move_create

> models::GatewayFirewallRule cloud_gateways_firewall_rules_move_create(gateway_id, id, gateway_firewall_move_request, revision)


Move a rule to a position, the others keeping their order. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**gateway_firewall_move_request** | [**GatewayFirewallMoveRequest**](GatewayFirewallMoveRequest.md) |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayFirewallRule**](GatewayFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_firewall_rules_partial_update

> models::GatewayFirewallRule cloud_gateways_firewall_rules_partial_update(gateway_id, id, patched_gateway_firewall_rule_write_request)


Change a rule's fields; a source or destination is replaced whole. Its position stays.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**patched_gateway_firewall_rule_write_request** | Option<[**PatchedGatewayFirewallRuleWriteRequest**](PatchedGatewayFirewallRuleWriteRequest.md)> |  |  |

### Return type

[**models::GatewayFirewallRule**](GatewayFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_firewall_rules_retrieve

> models::GatewayFirewallRule cloud_gateways_firewall_rules_retrieve(gateway_id, id)


A gateway's firewall rules, in evaluation order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

[**models::GatewayFirewallRule**](GatewayFirewallRule.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_list

> models::PaginatedGatewayList cloud_gateways_list(page)


Manage gateways

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayList**](PaginatedGatewayList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_create

> models::GatewayLbListener cloud_gateways_listeners_create(gateway_id, gateway_lb_listener_write_request)


Add a listener (spec 6.8): tcp and udp name a pool, http takes routes. Pending until applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**gateway_lb_listener_write_request** | Option<[**GatewayLbListenerWriteRequest**](GatewayLbListenerWriteRequest.md)> |  |  |

### Return type

[**models::GatewayLbListener**](GatewayLBListener.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_destroy

> models::GatewayOperation cloud_gateways_listeners_destroy(gateway_id, id, revision)


Delete a listener and its routes (spec 5.11): 202 with the record the agent's acknowledgement completes. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_list

> models::PaginatedGatewayLbListenerList cloud_gateways_listeners_list(gateway_id, page)


A gateway's load balancer listeners

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayLbListenerList**](PaginatedGatewayLBListenerList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_partial_update

> models::GatewayLbListener cloud_gateways_listeners_partial_update(gateway_id, id, patched_gateway_lb_listener_write_request)


Change a listener's pool, sources, comment or enabled flag; protocol, port and mode are fixed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**patched_gateway_lb_listener_write_request** | Option<[**PatchedGatewayLbListenerWriteRequest**](PatchedGatewayLbListenerWriteRequest.md)> |  |  |

### Return type

[**models::GatewayLbListener**](GatewayLBListener.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_retrieve

> models::GatewayLbListener cloud_gateways_listeners_retrieve(gateway_id, id)


A gateway's load balancer listeners

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

[**models::GatewayLbListener**](GatewayLBListener.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_routes_create

> models::GatewayRoute cloud_gateways_listeners_routes_create(gateway_id, listener_id, gateway_route_write_request)


Add a route (spec 5.6): hostnames and a path prefix to a pool. Pending until applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**listener_id** | **i32** |  | [required] |
**gateway_route_write_request** | Option<[**GatewayRouteWriteRequest**](GatewayRouteWriteRequest.md)> |  |  |

### Return type

[**models::GatewayRoute**](GatewayRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_routes_destroy

> models::GatewayOperation cloud_gateways_listeners_routes_destroy(gateway_id, id, listener_id, revision)


Delete a route (spec 5.11): 202 with the record the agent's acknowledgement completes. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**listener_id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_routes_list

> models::PaginatedGatewayRouteList cloud_gateways_listeners_routes_list(gateway_id, listener_id, page)


An http listener's routes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**listener_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayRouteList**](PaginatedGatewayRouteList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_routes_partial_update

> models::GatewayRoute cloud_gateways_listeners_routes_partial_update(gateway_id, id, listener_id, patched_gateway_route_write_request)


Change a route; hostnames are replaced whole.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**listener_id** | **i32** |  | [required] |
**patched_gateway_route_write_request** | Option<[**PatchedGatewayRouteWriteRequest**](PatchedGatewayRouteWriteRequest.md)> |  |  |

### Return type

[**models::GatewayRoute**](GatewayRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_listeners_routes_retrieve

> models::GatewayRoute cloud_gateways_listeners_routes_retrieve(gateway_id, id, listener_id)


An http listener's routes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**listener_id** | **i32** |  | [required] |

### Return type

[**models::GatewayRoute**](GatewayRoute.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_networks_create

> models::GatewayOperation cloud_gateways_networks_create(gateway_id, gateway_network_create_request)


Attach a network (spec 11.3): the operation reserves it and hotplugs the gateway's NIC.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**gateway_network_create_request** | [**GatewayNetworkCreateRequest**](GatewayNetworkCreateRequest.md) |  | [required] |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_networks_destroy

> models::GatewayOperation cloud_gateways_networks_destroy(gateway_id, network_id, revision, servers)


Detach a network (spec 11.3). Without the preview's revision and servers, 409 with a fresh preview.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**network_id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |
**servers** | Option<[**Vec<i32>**](I32.md)> | The preview's server ids. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_networks_list

> models::PaginatedGatewayNetworkList cloud_gateways_networks_list(gateway_id, page)


A gateway's private networks, by network id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayNetworkList**](PaginatedGatewayNetworkList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_networks_partial_update

> models::GatewayNetwork cloud_gateways_networks_partial_update(gateway_id, network_id, revision, servers, patched_gateway_network_update_request)


Switch a router network's SNAT (spec 6.4). Turning it off without the preview's revision and servers answers 409 with a fresh preview; the change is pending until the agent applies it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**network_id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |
**servers** | Option<[**Vec<i32>**](I32.md)> | The preview's server ids. |  |
**patched_gateway_network_update_request** | Option<[**PatchedGatewayNetworkUpdateRequest**](PatchedGatewayNetworkUpdateRequest.md)> |  |  |

### Return type

[**models::GatewayNetwork**](GatewayNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_networks_retrieve

> models::GatewayNetwork cloud_gateways_networks_retrieve(gateway_id, network_id)


A gateway's private networks, by network id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**network_id** | **i32** |  | [required] |

### Return type

[**models::GatewayNetwork**](GatewayNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_operations_list

> models::PaginatedGatewayOperationList cloud_gateways_operations_list(gateway_id, page)


A gateway's operations, newest first

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayOperationList**](PaginatedGatewayOperationList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_operations_retrieve

> models::GatewayOperation cloud_gateways_operations_retrieve(gateway_id, id)


A gateway's operations, newest first

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_partial_update

> models::Gateway cloud_gateways_partial_update(id, patched_gateway_update_request)


Rename the gateway or change its maintenance window. The name is not compiled, so no revision follows; a new window reaches the agent in a new revision (spec 11.4).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_gateway_update_request** | Option<[**PatchedGatewayUpdateRequest**](PatchedGatewayUpdateRequest.md)> |  |  |

### Return type

[**models::Gateway**](Gateway.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_backends_create

> models::GatewayBackend cloud_gateways_pools_backends_create(gateway_id, pool_id, gateway_backend_write_request)


Add a server to the pool (spec 5.5); it is pending until the agent applies its revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**pool_id** | **i32** |  | [required] |
**gateway_backend_write_request** | Option<[**GatewayBackendWriteRequest**](GatewayBackendWriteRequest.md)> |  |  |

### Return type

[**models::GatewayBackend**](GatewayBackend.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_backends_destroy

> models::GatewayOperation cloud_gateways_pools_backends_destroy(gateway_id, id, pool_id, revision)


Remove a server (spec 5.11): 202 with the record the agent's acknowledgement completes. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**pool_id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_backends_list

> models::PaginatedGatewayBackendList cloud_gateways_pools_backends_list(gateway_id, pool_id, page)


A backend pool's servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**pool_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayBackendList**](PaginatedGatewayBackendList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_backends_partial_update

> models::GatewayBackend cloud_gateways_pools_backends_partial_update(gateway_id, id, pool_id, patched_gateway_backend_write_request)


Change a server. The last enabled server of a pool in use cannot be disabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**pool_id** | **i32** |  | [required] |
**patched_gateway_backend_write_request** | Option<[**PatchedGatewayBackendWriteRequest**](PatchedGatewayBackendWriteRequest.md)> |  |  |

### Return type

[**models::GatewayBackend**](GatewayBackend.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_backends_retrieve

> models::GatewayBackend cloud_gateways_pools_backends_retrieve(gateway_id, id, pool_id)


A backend pool's servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**pool_id** | **i32** |  | [required] |

### Return type

[**models::GatewayBackend**](GatewayBackend.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_create

> models::GatewayBackendPool cloud_gateways_pools_create(gateway_id, gateway_backend_pool_write_request)


Add a backend pool (spec 5.5); it is pending until the agent applies its revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**gateway_backend_pool_write_request** | Option<[**GatewayBackendPoolWriteRequest**](GatewayBackendPoolWriteRequest.md)> |  |  |

### Return type

[**models::GatewayBackendPool**](GatewayBackendPool.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_destroy

> models::GatewayOperation cloud_gateways_pools_destroy(gateway_id, id, revision)


Delete an unused pool and its servers (spec 5.11): 202 with the record the agent's acknowledgement completes. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_list

> models::PaginatedGatewayBackendPoolList cloud_gateways_pools_list(gateway_id, page)


A gateway's backend pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayBackendPoolList**](PaginatedGatewayBackendPoolList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_partial_update

> models::GatewayBackendPool cloud_gateways_pools_partial_update(gateway_id, id, patched_gateway_backend_pool_write_request)


Change a pool; health_check takes the fields to change. Every listener and route on it must still fit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**patched_gateway_backend_pool_write_request** | Option<[**PatchedGatewayBackendPoolWriteRequest**](PatchedGatewayBackendPoolWriteRequest.md)> |  |  |

### Return type

[**models::GatewayBackendPool**](GatewayBackendPool.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_pools_retrieve

> models::GatewayBackendPool cloud_gateways_pools_retrieve(gateway_id, id)


A gateway's backend pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

[**models::GatewayBackendPool**](GatewayBackendPool.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_port_forwards_create

> models::GatewayForward cloud_gateways_port_forwards_create(gateway_id, gateway_forward_write_request)


Add a port forward (spec 6.7); it is pending until the agent applies its revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**gateway_forward_write_request** | [**GatewayForwardWriteRequest**](GatewayForwardWriteRequest.md) |  | [required] |

### Return type

[**models::GatewayForward**](GatewayForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_port_forwards_destroy

> models::GatewayOperation cloud_gateways_port_forwards_destroy(gateway_id, id, revision)


Delete a forward (spec 5.11): 202 with the record the agent's acknowledgement completes. Without the preview's revision, 409 with a fresh one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**revision** | Option<**i32**> | The gateway's desired revision from the preview; without it, a preview. |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_port_forwards_list

> models::PaginatedGatewayForwardList cloud_gateways_port_forwards_list(gateway_id, page)


A gateway's port forwards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**models::PaginatedGatewayForwardList**](PaginatedGatewayForwardList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_port_forwards_partial_update

> models::GatewayForward cloud_gateways_port_forwards_partial_update(gateway_id, id, patched_gateway_forward_write_request)


Change a forward's fields; public_ports and target are each replaced whole.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**patched_gateway_forward_write_request** | Option<[**PatchedGatewayForwardWriteRequest**](PatchedGatewayForwardWriteRequest.md)> |  |  |

### Return type

[**models::GatewayForward**](GatewayForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_port_forwards_retrieve

> models::GatewayForward cloud_gateways_port_forwards_retrieve(gateway_id, id)


A gateway's port forwards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **i32** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

[**models::GatewayForward**](GatewayForward.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_reenroll_create

> models::GatewayOperation cloud_gateways_reenroll_create(id, gateway_reenroll_request)


Re-enroll the gateway (spec 8.1 step 6): its agent token is revoked and it restarts to enroll again. Without the preview's revision, networks and servers, 409 with a fresh preview.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**gateway_reenroll_request** | Option<[**GatewayReenrollRequest**](GatewayReenrollRequest.md)> |  |  |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_retrieve

> models::Gateway cloud_gateways_retrieve(id)


Manage gateways

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Gateway**](Gateway.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_gateways_status_retrieve

> models::GatewayStatus cloud_gateways_status_retrieve(id)


The gateway's state and the customer part of its agent's last report (spec 9.5).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GatewayStatus**](GatewayStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::PublicIpv4 cloud_ipv4_create()


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicIpv4**](PublicIPv4.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::DetachIpv4Response cloud_ipv4_detach_create(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |

### Return type

[**models::DetachIpv4Response**](DetachIPv4Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::ReverseDns cloud_ipv4_rdns_create(id, reverse_dns_request)


Get or update reverse DNS (PTR) for this IPv4 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv4. | [required] |
**reverse_dns_request** | [**ReverseDnsRequest**](ReverseDnsRequest.md) |  | [required] |

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

> models::PublicIpv6 cloud_ipv6_create()


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicIpv6**](PublicIPv6.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::DetachIpv6Response cloud_ipv6_detach_create(id)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |

### Return type

[**models::DetachIpv6Response**](DetachIPv6Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::ReverseDns cloud_ipv6_rdns_create(id, reverse_dns_request)


Get or update reverse DNS (PTR) for this IPv6 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Public IPv6. | [required] |
**reverse_dns_request** | [**ReverseDnsRequest**](ReverseDnsRequest.md) |  | [required] |

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

> models::AddServerResponse cloud_private_networks_add_server_create(id, private_network_add_host_request)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**private_network_add_host_request** | [**PrivateNetworkAddHostRequest**](PrivateNetworkAddHostRequest.md) |  | [required] |

### Return type

[**models::AddServerResponse**](AddServerResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_apply_route_create

> models::GatewayOperation cloud_private_networks_apply_route_create(id, apply_route_request)


Re-render the named servers' routes and reboot them (spec 6.3); naming a server acknowledges its reboot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**apply_route_request** | [**ApplyRouteRequest**](ApplyRouteRequest.md) |  | [required] |

### Return type

[**models::GatewayOperation**](GatewayOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_create

> models::PrivateNetwork cloud_private_networks_create(private_network_request)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_network_request** | [**PrivateNetworkRequest**](PrivateNetworkRequest.md) |  | [required] |

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

> models::PrivateNetwork cloud_private_networks_partial_update(id, patched_private_network_update_request)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**patched_private_network_update_request** | Option<[**PatchedPrivateNetworkUpdateRequest**](PatchedPrivateNetworkUpdateRequest.md)> |  |  |

### Return type

[**models::PrivateNetwork**](PrivateNetwork.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_remove_server_create

> models::RemoveServerResponse cloud_private_networks_remove_server_create(id, private_network_remove_host_request)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**private_network_remove_host_request** | [**PrivateNetworkRemoveHostRequest**](PrivateNetworkRemoveHostRequest.md) |  | [required] |

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


## cloud_private_networks_route_status_retrieve

> models::RouteStatus cloud_private_networks_route_status_retrieve(id)


Each server's default route against the spec 6.2 rule, read from Proxmox, and the network's route applies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::RouteStatus**](RouteStatus.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_private_networks_update

> models::PrivateNetwork cloud_private_networks_update(id, private_network_update_request)


Manage private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this private network. | [required] |
**private_network_update_request** | Option<[**PrivateNetworkUpdateRequest**](PrivateNetworkUpdateRequest.md)> |  |  |

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

> Vec<models::BootIso> cloud_servers_boot_isos_list(id)


List the ISO catalog entries visible to this user and their package compatibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**Vec<models::BootIso>**](BootISO.md)

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

> models::ServerAddResponse cloud_servers_create(server_add_request)


Create new server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_add_request** | [**ServerAddRequest**](ServerAddRequest.md) |  | [required] |

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

> models::DestroyProtectionResponse cloud_servers_destroy_protection_create(id, destroy_protection_request)


Enable or disable destroy protection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**destroy_protection_request** | [**DestroyProtectionRequest**](DestroyProtectionRequest.md) |  | [required] |

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

> models::ServerUpgradeResponse cloud_servers_modify_package_create(id, server_product_upgrade_request)


Modify server package: downgrade available only for packages with the same disk size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**server_product_upgrade_request** | [**ServerProductUpgradeRequest**](ServerProductUpgradeRequest.md) |  | [required] |

### Return type

[**models::ServerUpgradeResponse**](ServerUpgradeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_monitoring_retrieve

> models::ServerMonitoring cloud_servers_monitoring_retrieve(id)


Show whether monitoring alerts are on for this server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::ServerMonitoring**](ServerMonitoring.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_monitoring_update

> models::ServerMonitoring cloud_servers_monitoring_update(id, server_monitoring_request)


Turn monitoring alerts on or off. Only an active server can change it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**server_monitoring_request** | [**ServerMonitoringRequest**](ServerMonitoringRequest.md) |  | [required] |

### Return type

[**models::ServerMonitoring**](ServerMonitoring.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_partial_update

> models::ServerDetail cloud_servers_partial_update(id, patched_server_detail_request)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**patched_server_detail_request** | Option<[**PatchedServerDetailRequest**](PatchedServerDetailRequest.md)> |  |  |

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


Queue a power action and return the state observed before queueing. If the state cannot be read, the action is refused with HTTP 503.

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


Read the server's power state and the reason if it is unavailable.

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

> models::PublicInterface cloud_servers_public_interface_create(id, public_interface_request)


Public interface

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**public_interface_request** | Option<[**PublicInterfaceRequest**](PublicInterfaceRequest.md)> |  |  |

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

> models::SnapshotCreateQueued cloud_servers_snapshots_create(id, snapshot_create_request)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**snapshot_create_request** | [**SnapshotCreateRequest**](SnapshotCreateRequest.md) |  | [required] |

### Return type

[**models::SnapshotCreateQueued**](SnapshotCreateQueued.md)

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

> Vec<models::Snapshot> cloud_servers_snapshots_list(id)


List snapshots for this server or queue a new snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**Vec<models::Snapshot>**](Snapshot.md)

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


## cloud_servers_traffic_retrieve

> models::ServerTrafficResponse cloud_servers_traffic_retrieve(id)


Get this month's traffic usage for a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**models::ServerTrafficResponse**](ServerTrafficResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_servers_update

> models::ServerDetail cloud_servers_update(id, server_detail_request)


Cloud servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**server_detail_request** | Option<[**ServerDetailRequest**](ServerDetailRequest.md)> |  |  |

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

> models::Volume cloud_servers_volumes_create(server_id, volume_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_request** | [**VolumeRequest**](VolumeRequest.md) |  | [required] |

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

> models::Volume cloud_servers_volumes_partial_update(server_id, volume_id, patched_volume_update_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_id** | **String** |  | [required] |
**patched_volume_update_request** | Option<[**PatchedVolumeUpdateRequest**](PatchedVolumeUpdateRequest.md)> |  |  |

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

> models::Volume cloud_servers_volumes_update(server_id, volume_id, volume_update_request)


Adds :class:`~account.iam_enforcement.IAMActionPermission` as an intersection with the route's existing permission classes (spec §6).  Detail routes (``self.detail``) defer the role/scope check to ``has_object_permission`` so the account-scoped ``get_object`` answers 404 for foreign IDs before any role denial; every other route enforces in ``has_permission``. A detail action that never calls ``get_object`` would skip enforcement — the route probes pin the denial for each route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**volume_id** | **String** |  | [required] |
**volume_update_request** | [**VolumeUpdateRequest**](VolumeUpdateRequest.md) |  | [required] |

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

> models::AttachVolume cloud_volumes_attach_create(id, attach_volume_request)


Attach existing volume to a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**attach_volume_request** | [**AttachVolumeRequest**](AttachVolumeRequest.md) |  | [required] |

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

> models::DetachVolume cloud_volumes_detach_create(id)


Detach volume from server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |

### Return type

[**models::DetachVolume**](DetachVolume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::Volume cloud_volumes_partial_update(id, patched_volume_update_request)


Volumes management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**patched_volume_update_request** | Option<[**PatchedVolumeUpdateRequest**](PatchedVolumeUpdateRequest.md)> |  |  |

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

> models::Volume cloud_volumes_update(id, volume_update_request)


Volumes management

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this storage. | [required] |
**volume_update_request** | [**VolumeUpdateRequest**](VolumeUpdateRequest.md) |  | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

