# CastWithInteractionsAndConversationsRef

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** |  | 
**hash** | **String** |  | 
**parent_hash** | Option<**String**> |  | 
**parent_url** | Option<**String**> |  | 
**root_parent_url** | Option<**String**> |  | 
**parent_author** | [**models::CastEmbeddedParentAuthor**](CastEmbedded_parent_author.md) |  | 
**author** | [**models::User**](User.md) |  | 
**app** | Option<[**models::UserDehydrated**](UserDehydrated.md)> |  | [optional]
**text** | **String** |  | 
**timestamp** | **String** |  | 
**embeds** | [**Vec<models::Embed>**](Embed.md) |  | 
**r#type** | Option<[**models::CastNotificationType**](CastNotificationType.md)> |  | [optional]
**frames** | Option<[**Vec<models::Frame>**](Frame.md)> |  | [optional]
**reactions** | [**models::CastWithInteractionsReactions**](CastWithInteractionsReactions.md) |  | 
**replies** | [**models::CastWithInteractionsReplies**](CastWithInteractionsReplies.md) |  | 
**thread_hash** | Option<**String**> |  | 
**mentioned_profiles** | [**Vec<models::User>**](User.md) |  | 
**mentioned_profiles_ranges** | [**Vec<models::TextRange>**](TextRange.md) | Positions within the text (inclusive start, exclusive end) where each mention occurs. Each index within this list corresponds to the same-numbered index in the mentioned_profiles list.  | 
**mentioned_channels** | [**Vec<models::ChannelDehydrated>**](ChannelDehydrated.md) |  | 
**mentioned_channels_ranges** | [**Vec<models::TextRange>**](TextRange.md) | Positions within the text (inclusive start, exclusive end) where each mention occurs. Each index within this list corresponds to the same-numbered index in the mentioned_channels list.  | 
**channel** | Option<[**models::ChannelOrChannelDehydrated**](ChannelOrChannelDehydrated.md)> |  | 
**viewer_context** | Option<[**models::CastViewerContext**](CastViewerContext.md)> |  | [optional]
**author_channel_context** | Option<[**models::ChannelUserContext**](ChannelUserContext.md)> |  | [optional]
**direct_replies** | [**Vec<serde_json::Value>**](serde_json::Value.md) | note: This is recursive. It contains the direct replies to the cast and their direct replies up to n reply_depth. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


