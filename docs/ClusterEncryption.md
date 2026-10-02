# ClusterEncryption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **String** |  | [readonly]
**status** | [**models::ClusterEncryptionStatusEnum**](ClusterEncryptionStatusEnum.md) |  | [readonly]
**changed_at** | Option<**String**> |  | [readonly]
**verified_at** | Option<**String**> |  | [readonly]
**restart_required** | **bool** |  | [readonly]
**restart_required_at** | Option<**String**> |  | [readonly]
**restart_checked_at** | Option<**String**> |  | [readonly]
**stale_pod_count** | Option<**i32**> |  | [readonly]
**reason** | **Reason** |  (enum: feature_unavailable, cluster_operation_in_progress, resource_not_active, cluster_not_provisioned, invalid_encryption_mode, workload_restart_ack_required, encryption_state_unknown, encryption_already_in_requested_mode, reconcile_requires_staff, reconcile_requires_unknown_state, cilium_version_mismatch, kernel_wireguard_unavailable, kernel_nodes_unknown, helm_upgrade_failed, helm_rollback_failed, cilium_rollout_failed, cilium_node_key_missing, cilium_peer_verification_failed, cilium_agent_unreadable, cilium_agent_state_mismatch, cilium_encryption_not_active, cilium_encryption_still_active, cilium_nodes_unknown, cilium_mode_unsupported, cilium_verification_misconfigured, restart_recheck_failed, platform_restart_incomplete, encryption_task_aborted, dispatch_pending, ) | [readonly]
**error** | **String** |  | [readonly]
**per_node** | **std::collections::HashMap<String, serde_json::Value>** | Evidence from the NEWEST operation, which may not have any yet.  A freshly queued operation carries an empty ``verification_result``, so this blanks the moment a toggle is admitted while ``mode`` and ``status`` still describe the last verified state. An empty map therefore means \"no evidence from the current operation\", NEVER \"verification failed\" -- read ``operation.status`` to tell them apart. Showing the previous operation's rows instead would label evidence for one mode as evidence for another. | [readonly]
**operation** | Option<[**models::ClusterEncryptionOperation**](ClusterEncryptionOperation.md)> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


