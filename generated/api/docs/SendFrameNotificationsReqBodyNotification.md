# SendFrameNotificationsReqBodyNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title of the notification. Must be between 1 and 32 characters. | 
**body** | **String** | The body of the notification. Must be between 1 and 128 characters. | 
**target_url** | **String** | The target URL to open when the user clicks the notification. Must be a valid URL. | 
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | An optional UUID for the notification, used as an idempotency key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


