# SendFrameNotificationsReqBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_fids** | **Vec<i32>** | An array of target FIDs to whom the notifications should be sent. Each FID must be a positive integer. Pass an empty array to send notifications to all FIDs with notifications enabled for the mini app. | 
**notification** | [**models::SendFrameNotificationsReqBodyNotification**](SendFrameNotificationsReqBody_notification.md) |  | 
**filters** | Option<[**models::SendFrameNotificationsReqBodyFilters**](SendFrameNotificationsReqBody_filters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


