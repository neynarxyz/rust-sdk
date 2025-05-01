# \ReactionsApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_cast_reactions**](ReactionsApi.md#fetch_cast_reactions) | **GET** /v1/reactionsByCast | On cast
[**fetch_reactions_by_target**](ReactionsApi.md#fetch_reactions_by_target) | **GET** /v1/reactionsByTarget | To a target URL
[**fetch_user_reactions**](ReactionsApi.md#fetch_user_reactions) | **GET** /v1/reactionsByFid | By FID
[**lookup_reaction_by_id**](ReactionsApi.md#lookup_reaction_by_id) | **GET** /v1/reactionById | By FID or cast



## fetch_cast_reactions

> models::FetchCastReactions200Response fetch_cast_reactions(target_fid, target_hash, reaction_type, page_size, reverse, page_token)
On cast

Retrieve all reactions (likes or recasts) on a specific cast in the Farcaster network. The cast is identified by its creator's FID and unique hash. This endpoint helps track engagement metrics and user interactions with specific content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_fid** | **i32** | The FID of the cast's creator. Required to uniquely identify the cast that received the reactions. Must be used in conjunction with target_hash. | [required] |
**target_hash** | **String** | The unique hash identifier of the cast that received the reactions. This is a 40-character hexadecimal string prefixed with '0x' that uniquely identifies the cast within the creator's posts. Must be used with target_fid. | [required] |
**reaction_type** | [**ReactionType**](.md) |  | [required] |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchCastReactions200Response**](fetch_cast_reactions_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_reactions_by_target

> models::FetchCastReactions200Response fetch_reactions_by_target(url, reaction_type, page_size, reverse, page_token)
To a target URL

Fetch all reactions of a specific type (like or recast) that target a given URL. This endpoint is useful for tracking engagement with content across the Farcaster network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Target URL starting with 'chain://'. | [required] |
**reaction_type** | Option<[**ReactionType**](.md)> |  |  |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchCastReactions200Response**](fetch_cast_reactions_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_reactions

> models::FetchCastReactions200Response fetch_user_reactions(fid, reaction_type, page_size, reverse, page_token)
By FID

Fetch reactions by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the reaction's creator | [required] |
**reaction_type** | [**ReactionType**](.md) |  | [required] |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchCastReactions200Response**](fetch_cast_reactions_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_reaction_by_id

> models::Reaction lookup_reaction_by_id(fid, target_fid, target_hash, reaction_type)
By FID or cast

Lookup a reaction by its FID or cast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the reaction's creator | [required] |
**target_fid** | **i32** | The FID of the cast's creator | [required] |
**target_hash** | **String** | The cast's hash | [required] |
**reaction_type** | [**ReactionType**](.md) |  | [required] |

### Return type

[**models::Reaction**](Reaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

