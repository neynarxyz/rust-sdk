# Channel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**url** | **String** |  | 
**object** | **String** |  | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**description_mentioned_profiles** | Option<[**Vec<models::UserDehydrated>**](UserDehydrated.md)> |  | [optional]
**description_mentioned_profiles_ranges** | Option<[**Vec<models::TextRange>**](TextRange.md)> | Positions within the text (inclusive start, exclusive end) where each mention occurs. | [optional]
**created_at** | Option<**f64**> | Epoch timestamp in seconds. | [optional]
**follower_count** | Option<**f64**> | Number of followers the channel has. | [optional]
**external_link** | Option<[**models::ChannelExternalLink**](Channel_external_link.md)> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**parent_url** | Option<**String**> |  | [optional]
**lead** | Option<[**models::User**](User.md)> |  | [optional]
**moderator_fids** | Option<**Vec<i32>**> |  | [optional]
**member_count** | Option<**i32**> |  | [optional]
**moderator** | Option<[**models::User**](User.md)> | Use `lead` instead. | [optional]
**pinned_cast_hash** | Option<**String**> | Cast Hash | [optional][default to 0xfe90f9de682273e05b201629ad2338bdcd89b6be]
**hosts** | Option<[**Vec<models::User>**](User.md)> |  | [optional]
**viewer_context** | Option<[**models::ChannelUserContext**](ChannelUserContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


