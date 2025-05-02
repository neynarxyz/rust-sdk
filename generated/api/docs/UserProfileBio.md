# UserProfileBio

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** |  | 
**mentioned_profiles** | Option<[**Vec<models::UserDehydrated>**](UserDehydrated.md)> |  | [optional]
**mentioned_profiles_ranges** | Option<[**Vec<models::TextRange>**](TextRange.md)> | Positions within the text (inclusive start, exclusive end) where each mention occurs. Each index within this list corresponds to the same-numbered index in the mentioned_profiles list.  | [optional]
**mentioned_channels** | Option<[**Vec<models::ChannelDehydrated>**](ChannelDehydrated.md)> |  | [optional]
**mentioned_channels_ranges** | Option<[**Vec<models::TextRange>**](TextRange.md)> | Positions within the text (inclusive start, exclusive end) where each mention occurs. Each index within this list corresponds to the same-numbered index in the mentioned_channels list.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


