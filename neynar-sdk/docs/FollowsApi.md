# \FollowsApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_follow_suggestions**](FollowsApi.md#fetch_follow_suggestions) | **GET** /farcaster/following/suggested | Suggest Follows
[**fetch_relevant_followers**](FollowsApi.md#fetch_relevant_followers) | **GET** /farcaster/followers/relevant | Relevant followers
[**fetch_user_followers**](FollowsApi.md#fetch_user_followers) | **GET** /farcaster/followers | Followers
[**fetch_user_following**](FollowsApi.md#fetch_user_following) | **GET** /farcaster/following | Following



## fetch_follow_suggestions

> models::UsersResponse fetch_follow_suggestions(fid, viewer_fid, limit, x_neynar_experimental)
Suggest Follows

Fetch a list of suggested users to follow. Used to help users discover new users to follow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of the user whose following you want to fetch. | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a list of users that respects this user's mutes and blocks and includes `viewer_context`. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UsersResponse**](UsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_relevant_followers

> models::RelevantFollowersResponse fetch_relevant_followers(target_fid, viewer_fid, x_neynar_experimental)
Relevant followers

Returns a list of relevant followers for a specific FID. This usually shows on a profile as \"X, Y and Z follow this user\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_fid** | **i32** | User who's profile you are looking at | [required] |
**viewer_fid** | **i32** | The FID of the user to customize this response for. Providing this will also return a list of followers that respects this user's mutes and blocks and includes `viewer_context`. | [required] |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::RelevantFollowersResponse**](RelevantFollowersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_followers

> models::FollowersResponse fetch_user_followers(fid, viewer_fid, sort_type, limit, cursor, x_neynar_experimental)
Followers

Returns a list of followers for a specific FID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | User who's profile you are looking at | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a list of followers that respects this user's mutes and blocks and includes `viewer_context`. |  |
**sort_type** | Option<[**FollowSortType**](.md)> | Sort type for fetch followers. Default is `desc_chron` |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FollowersResponse**](FollowersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_following

> models::FollowersResponse fetch_user_following(fid, viewer_fid, sort_type, limit, cursor, x_neynar_experimental)
Following

Fetch a list of users who a given user is following. Can optionally include a viewer_fid and sort_type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of the user whose following you want to fetch. | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a list of users that respects this user's mutes and blocks and includes `viewer_context`. |  |
**sort_type** | Option<[**FollowSortType**](.md)> | Optional parameter to sort the users based on different criteria. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::FollowersResponse**](FollowersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

