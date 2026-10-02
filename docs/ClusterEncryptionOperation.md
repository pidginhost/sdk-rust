# ClusterEncryptionOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**kind** | **String** |  | [readonly]
**status** | **String** |  | [readonly]
**requested_mode** | **String** |  | [readonly]
**previous_mode** | **String** |  | [readonly]
**reason** | **Reason** |  (enum: feature_unavailable, cluster_operation_in_progress, resource_not_active, cluster_not_provisioned, invalid_encryption_mode, workload_restart_ack_required, encryption_state_unknown, encryption_already_in_requested_mode, reconcile_requires_staff, reconcile_requires_unknown_state, cilium_version_mismatch, kernel_wireguard_unavailable, kernel_nodes_unknown, helm_upgrade_failed, helm_rollback_failed, cilium_rollout_failed, cilium_node_key_missing, cilium_peer_verification_failed, cilium_agent_unreadable, cilium_agent_state_mismatch, cilium_encryption_not_active, cilium_encryption_still_active, cilium_nodes_unknown, cilium_mode_unsupported, cilium_verification_misconfigured, restart_recheck_failed, platform_restart_incomplete, encryption_task_aborted, dispatch_pending, ) | [readonly]
**message** | **String** |  | [readonly]
**override_unverifiable** | **bool** |  | [readonly]
**request_id** | **String** |  | [readonly]
**created_at** | **String** |  | [readonly]
**finished_at** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


