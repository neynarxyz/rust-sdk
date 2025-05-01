# Notification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** |  | 
**most_recent_timestamp** | **String** |  | 
**r#type** | **String** |  | 
**seen** | **bool** |  | 
**follows** | Option<[**Vec<models::Follower>**](Follower.md)> |  | [optional]
**cast** | Option<[**models::CastWithInteractions**](CastWithInteractions.md)> |  | [optional]
**reactions** | Option<[**Vec<models::ReactionWithUserInfo>**](ReactionWithUserInfo.md)> |  | [optional]
**count** | Option<**i32**> | The number of notifications of this(follows, likes, recast) type bundled in a single notification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


