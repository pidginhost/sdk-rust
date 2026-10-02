# NodeOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**kind** | [**models::NodeOperationKindEnum**](NodeOperationKindEnum.md) |  | [readonly]
**source** | [**models::NodeOperationSourceEnum**](NodeOperationSourceEnum.md) |  | [readonly]
**target_hostname** | **String** |  | [readonly]
**status** | [**models::NodeOperationStatusEnum**](NodeOperationStatusEnum.md) |  | [readonly]
**reason** | **String** |  | [readonly]
**message** | **String** |  | [readonly]
**bypass_pdb** | **bool** |  | [readonly]
**delete_unmanaged_pods** | **bool** |  | [readonly]
**local_data_loss_accepted** | **bool** |  | [readonly]
**bypass_pdb_confirmed_at** | Option<**String**> |  | [readonly]
**unmanaged_pods_confirmed_at** | Option<**String**> |  | [readonly]
**actor_label** | **String** | Who requested the operation (user email or staff name). Never token material. | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | **String** |  | [readonly]
**finished_at** | Option<**String**> |  | [readonly]
**allowed_actions** | **Vec<String>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


