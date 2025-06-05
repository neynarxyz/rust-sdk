# GetNotificationCampaignStats200ResponseNotificationCampaignsInnerStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intended_recipient_notification_token_count** | Option<**i32**> | The total number of notification tokens for intended recipients. | [optional]
**intended_recipient_app_fids** | Option<**Vec<i32>**> | An array of Farcaster FIDs of intended recipient applications. | [optional]
**successful_sends** | Option<**i32**> | The number of notifications successfully sent. | [optional]
**successful_sends_by_app_fid** | Option<**std::collections::HashMap<String, i32>**> | A record mapping app FIDs (as strings) to the number of successful sends for that app. | [optional]
**total_opens** | Option<**i32**> | The total number of times notifications from this campaign have been opened. | [optional]
**total_opens_by_app_fid** | Option<**std::collections::HashMap<String, i32>**> | A record mapping app FIDs (as strings) to the number of opens for that app. | [optional]
**unique_opens** | Option<**i32**> | The number of unique recipients who opened a notification from this campaign. | [optional]
**unique_opens_by_app_fid** | Option<**std::collections::HashMap<String, i32>**> | A record mapping app FIDs (as strings) to the number of unique opens for that app. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


