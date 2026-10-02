# PoolRemovalJournal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**kind** | [**models::PoolRemovalJournalKindEnum**](PoolRemovalJournalKindEnum.md) |  | [readonly]
**status** | [**models::PoolRemovalJournalStatusEnum**](PoolRemovalJournalStatusEnum.md) |  | [readonly]
**reason** | **String** |  | [readonly]
**message** | **String** |  | [readonly]
**requested_pool_size** | Option<**i32**> |  | [readonly]
**local_data_loss_accepted** | **bool** |  | [readonly]
**actor_label** | **String** | Who requested the removal (user email or staff name). Never token material. | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | **String** |  | [readonly]
**finished_at** | Option<**String**> |  | [readonly]
**items** | [**Vec<models::PoolRemovalItem>**](PoolRemovalItem.md) |  | [readonly]
**allowed_actions** | **Vec<String>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


