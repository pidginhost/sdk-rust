# SendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_address** | **String** |  | 
**to** | **Vec<String>** |  | 
**cc** | Option<**Vec<String>**> |  | [optional]
**bcc** | Option<**Vec<String>**> |  | [optional]
**reply_to** | Option<**String**> |  | [optional]
**subject** | **String** |  | 
**html_body** | Option<**String**> |  | [optional]
**plain_body** | Option<**String**> |  | [optional]
**headers** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**track_opens** | Option<**bool**> |  | [optional][default to false]
**track_clicks** | Option<**bool**> |  | [optional][default to false]
**attachments** | Option<[**Vec<models::AttachmentRequest>**](AttachmentRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


