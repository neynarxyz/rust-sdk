# SendFrameNotificationsReqBodyFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exclude_fids** | Option<**Vec<i32>**> | Only send notifications to users who are not in the given FIDs. | [optional]
**following_fid** | Option<**i32**> | Only send notifications to users who follow the given FID. | [optional]
**minimum_user_score** | Option<**f64**> | Only send notifications to users with a score greater than or equal to this value. | [optional]
**near_location** | Option<[**models::Location**](Location.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


