# \FeedApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_casts_for_user**](FeedApi.md#fetch_casts_for_user) | **GET** /farcaster/feed/user/casts | Chronologically
[**fetch_feed**](FeedApi.md#fetch_feed) | **GET** /farcaster/feed | By filters
[**fetch_feed_by_channel_ids**](FeedApi.md#fetch_feed_by_channel_ids) | **GET** /farcaster/feed/channels | By channel IDs
[**fetch_feed_by_parent_urls**](FeedApi.md#fetch_feed_by_parent_urls) | **GET** /farcaster/feed/parent_urls | By parent URLs
[**fetch_feed_for_you**](FeedApi.md#fetch_feed_for_you) | **GET** /farcaster/feed/for_you | For you
[**fetch_frames_only_feed**](FeedApi.md#fetch_frames_only_feed) | **GET** /farcaster/feed/frames | Casts with mini apps
[**fetch_popular_casts_by_user**](FeedApi.md#fetch_popular_casts_by_user) | **GET** /farcaster/feed/user/popular | 10 most popular casts
[**fetch_replies_and_recasts_for_user**](FeedApi.md#fetch_replies_and_recasts_for_user) | **GET** /farcaster/feed/user/replies_and_recasts | Replies and recasts
[**fetch_trending_feed**](FeedApi.md#fetch_trending_feed) | **GET** /farcaster/feed/trending | Trending feeds
[**fetch_user_following_feed**](FeedApi.md#fetch_user_following_feed) | **GET** /farcaster/feed/following | Following



## fetch_casts_for_user

> models::FeedResponse fetch_casts_for_user(fid, app_fid, viewer_fid, limit, cursor, include_replies, parent_url, channel_id, x_neynar_experimental)
Chronologically

Fetch casts for a given user FID in reverse chronological order. Also allows filtering by parent_url and channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of user whose recent casts you want to fetch | [required] |
**app_fid** | Option<**i32**> | Optionally filter to casts created via a specific app FID, e.g. 9152 for Warpcast |  |
**viewer_fid** | Option<**i32**> | FID of the user viewing the feed |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor |  |
**include_replies** | Option<**bool**> | Include reply casts by the author in the response, true by default |  |[default to true]
**parent_url** | Option<**String**> | Parent URL to filter the feed; mutually exclusive with channel_id |  |
**channel_id** | Option<**String**> | Channel ID to filter the feed; mutually exclusive with parent_url |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_feed

> models::FeedResponse fetch_feed(feed_type, filter_type, fid, fids, parent_url, channel_id, members_only, embed_url, embed_types, with_recasts, limit, cursor, viewer_fid, x_neynar_experimental)
By filters

Fetch casts based on filters. Ensure setting the correct parameters based on the feed_type and filter_type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_type** | [**FeedType**](.md) | Defaults to following (requires FID or address). If set to filter (requires filter_type) | [required] |
**filter_type** | Option<[**FilterType**](.md)> | Used when feed_type=filter. Can be set to FIDs (requires FIDs) or parent_url (requires parent_url) or channel_id (requires channel_id) |  |
**fid** | Option<**i32**> | (Optional) FID of user whose feed you want to create. By default, the API expects this field, except if you pass a filter_type |  |
**fids** | Option<**String**> | Used when filter_type=FIDs . Create a feed based on a list of FIDs. Max array size is 100. Requires feed_type and filter_type. |  |
**parent_url** | Option<**String**> | Used when filter_type=parent_url can be used to fetch content under any parent url e.g. FIP-2 channels on Warpcast. Requires feed_type and filter_type. |  |
**channel_id** | Option<**String**> | Used when filter_type=channel_id can be used to fetch casts under a channel. Requires feed_type and filter_type. |  |
**members_only** | Option<**bool**> | Used when filter_type=channel_id. Only include casts from members of the channel. True by default. |  |[default to true]
**embed_url** | Option<**String**> | Used when filter_type=embed_url. Casts with embedded URLs prefixed by this embed_url param will be returned. We normalize your given URL prefix and prepend 'https://' if no protocol is included. Requires feed_type and filter_type. |  |
**embed_types** | Option<[**Vec<models::EmbedType>**](models::EmbedType.md)> | Used when filter_type=embed_types can be used to fetch all casts with matching content types. Requires feed_type and filter_type. |  |
**with_recasts** | Option<**bool**> | Include recasts in the response, true by default |  |[default to true]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_feed_by_channel_ids

> models::FeedResponse fetch_feed_by_channel_ids(channel_ids, with_recasts, viewer_fid, with_replies, members_only, fids, limit, cursor, should_moderate, x_neynar_experimental)
By channel IDs

Fetch feed based on channel IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_ids** | **String** | Comma separated list of up to 10 channel IDs e.g. neynar,farcaster | [required] |
**with_recasts** | Option<**bool**> | Include recasts in the response, true by default |  |[default to true]
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**with_replies** | Option<**bool**> | Include replies in the response, false by default |  |[default to false]
**members_only** | Option<**bool**> | Only include casts from members of the channel. True by default. |  |[default to true]
**fids** | Option<**String**> | Comma separated list of FIDs to filter the feed by, up to 10 at a time |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**should_moderate** | Option<**bool**> | If true, only casts that have been liked by the moderator (if one exists) will be returned. |  |[default to false]
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_feed_by_parent_urls

> models::FeedResponse fetch_feed_by_parent_urls(parent_urls, with_recasts, viewer_fid, with_replies, limit, cursor, x_neynar_experimental)
By parent URLs

Fetch feed based on parent URLs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_urls** | **String** | Comma separated list of parent_urls | [required] |
**with_recasts** | Option<**bool**> | Include recasts in the response, true by default |  |[default to true]
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**with_replies** | Option<**bool**> | Include replies in the response, false by default |  |[default to false]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_feed_for_you

> models::FeedResponse fetch_feed_for_you(fid, viewer_fid, provider, limit, cursor, provider_metadata, x_neynar_experimental)
For you

Fetch a personalized For You feed for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of user whose feed you want to create | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**provider** | Option<[**ForYouProvider**](.md)> |  |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**provider_metadata** | Option<**String**> | provider_metadata is a URI-encoded stringified JSON object that can be used to pass additional metadata to the provider. Only available for mbd provider right now. See [here](https://docs.neynar.com/docs/feed-for-you-w-external-providers) on how to use.  |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_frames_only_feed

> models::FeedResponse fetch_frames_only_feed(limit, viewer_fid, cursor, x_neynar_experimental)
Casts with mini apps

Fetch feed of casts with mini apps, reverse chronological order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_popular_casts_by_user

> models::BulkCastsResponse fetch_popular_casts_by_user(fid, viewer_fid)
10 most popular casts

Fetch 10 most popular casts for a given user FID; popularity based on replies, likes and recasts; sorted by most popular first

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of user whose feed you want to create | [required] |
**viewer_fid** | Option<**i32**> |  |  |

### Return type

[**models::BulkCastsResponse**](BulkCastsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_replies_and_recasts_for_user

> models::FeedResponse fetch_replies_and_recasts_for_user(fid, filter, limit, cursor, viewer_fid)
Replies and recasts

Fetch recent replies and recasts for a given user FID; sorted by most recent first

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of user whose replies and recasts you want to fetch | [required] |
**filter** | Option<**String**> | filter to fetch only replies or recasts |  |[default to all]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_trending_feed

> models::FeedResponse fetch_trending_feed(limit, cursor, viewer_fid, time_window, channel_id, parent_url, provider, provider_metadata, x_neynar_experimental)
Trending feeds

Fetch trending casts or on the global feed or channels feeds. 7d time window available for channel feeds only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 10]
**cursor** | Option<**String**> | Pagination cursor |  |
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**time_window** | Option<[**TrendingTimeWindow**](.md)> | Time window for trending casts (7d window for channel feeds only) |  |
**channel_id** | Option<**String**> | Channel ID to filter trending casts. Less active channels might have no casts in the time window selected. Provide either `channel_id` or `parent_url`, not both. |  |
**parent_url** | Option<**String**> | Parent URL to filter trending casts. Less active channels might have no casts in the time window selected. Provide either `channel_id` or `parent_url`, not both. |  |
**provider** | Option<[**FeedTrendingProvider**](.md)> | The provider of the trending casts feed. |  |
**provider_metadata** | Option<**String**> | provider_metadata is a URI-encoded stringified JSON object that can be used to pass additional metadata to the provider. Only available for mbd provider right now. See [here](https://docs.neynar.com/docs/feed-for-you-w-external-providers) on how to use.  |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_following_feed

> models::FeedResponse fetch_user_following_feed(fid, viewer_fid, with_recasts, limit, cursor, x_neynar_experimental)
Following

Fetch feed based on who a user is following

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of user whose feed you want to create | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a feed that respects this user's mutes and blocks and includes `viewer_context`. |  |
**with_recasts** | Option<**bool**> | Include recasts in the response, true by default |  |[default to true]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FeedResponse**](FeedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

