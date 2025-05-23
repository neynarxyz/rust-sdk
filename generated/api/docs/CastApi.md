# \CastApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_cast**](CastApi.md#delete_cast) | **DELETE** /farcaster/cast | Delete a cast
[**fetch_bulk_casts**](CastApi.md#fetch_bulk_casts) | **GET** /farcaster/casts | Bulk fetch casts
[**fetch_composer_actions**](CastApi.md#fetch_composer_actions) | **GET** /farcaster/cast/composer_actions/list | Fetch composer actions
[**fetch_embedded_url_metadata**](CastApi.md#fetch_embedded_url_metadata) | **GET** /farcaster/cast/embed/crawl | Embedded URL metadata
[**lookup_cast_by_hash_or_warpcast_url**](CastApi.md#lookup_cast_by_hash_or_warpcast_url) | **GET** /farcaster/cast | By hash or URL
[**lookup_cast_conversation**](CastApi.md#lookup_cast_conversation) | **GET** /farcaster/cast/conversation | Conversation for a cast
[**publish_cast**](CastApi.md#publish_cast) | **POST** /farcaster/cast | Post a cast
[**search_casts**](CastApi.md#search_casts) | **GET** /farcaster/cast/search | Search for casts



## delete_cast

> models::OperationResponse delete_cast(delete_cast_req_body)
Delete a cast

Delete an existing cast. \\ (In order to delete a cast `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_cast_req_body** | [**DeleteCastReqBody**](DeleteCastReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_bulk_casts

> models::CastsResponse fetch_bulk_casts(casts, viewer_fid, sort_type, x_neynar_experimental)
Bulk fetch casts

Fetch multiple casts using their respective hashes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**casts** | **String** | Hashes of the cast to be retrived (Comma separated, no spaces) | [required] |
**viewer_fid** | Option<**i32**> | adds viewer_context to cast object to show whether viewer has liked or recasted the cast. |  |
**sort_type** | Option<**String**> | Optional parameter to sort the casts based on different criteria |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::CastsResponse**](CastsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_composer_actions

> models::CastComposerActionsListResponse fetch_composer_actions(list, limit, cursor)
Fetch composer actions

Fetches all composer actions on Warpcast. You can filter by top or featured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | [**CastComposerType**](.md) | Type of list to fetch. | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::CastComposerActionsListResponse**](CastComposerActionsListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_embedded_url_metadata

> models::CastEmbedCrawlResponse fetch_embedded_url_metadata(url)
Embedded URL metadata

Crawls the given URL and returns metadata useful when embedding the URL in a cast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | URL to crawl metadata of | [required] |

### Return type

[**models::CastEmbedCrawlResponse**](CastEmbedCrawlResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_cast_by_hash_or_warpcast_url

> models::CastResponse lookup_cast_by_hash_or_warpcast_url(identifier, r#type, viewer_fid, x_neynar_experimental)
By hash or URL

Gets information about an individual cast by passing in a Farcaster web URL or cast hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Cast identifier (Its either a url or a hash) | [required] |
**r#type** | [**CastParamType**](.md) |  | [required] |
**viewer_fid** | Option<**i32**> | adds viewer_context to cast object to show whether viewer has liked or recasted the cast. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::CastResponse**](CastResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_cast_conversation

> models::Conversation lookup_cast_conversation(identifier, r#type, reply_depth, include_chronological_parent_casts, viewer_fid, sort_type, fold, limit, cursor, x_neynar_experimental)
Conversation for a cast

Gets all casts related to a conversation surrounding a cast by passing in a cast hash or Farcaster URL. Includes all the ancestors of a cast up to the root parent in a chronological order. Includes all direct_replies to the cast up to the reply_depth specified in the query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Cast identifier (Its either a url or a hash) | [required] |
**r#type** | [**CastParamType**](.md) |  | [required] |
**reply_depth** | Option<**i32**> | The depth of replies in the conversation that will be returned (default 2) |  |[default to 2]
**include_chronological_parent_casts** | Option<**bool**> | Include all parent casts in chronological order |  |[default to false]
**viewer_fid** | Option<**i32**> | Providing this will return a conversation that respects this user's mutes and blocks and includes `viewer_context`. |  |
**sort_type** | Option<[**CastConversationSortType**](.md)> | Sort type for the ordering of descendants. Default is `chron` |  |
**fold** | Option<**String**> | Show conversation above or below the fold. Lower quality responses are hidden below the fold. Not passing in a value shows the full conversation without any folding. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::Conversation**](Conversation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_cast

> models::PostCastResponse publish_cast(post_cast_req_body)
Post a cast

Posts a cast or cast reply. Works with mentions and embeds.   (In order to post a cast `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_cast_req_body** | [**PostCastReqBody**](PostCastReqBody.md) |  | [required] |

### Return type

[**models::PostCastResponse**](PostCastResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_casts

> models::CastsSearchResponse search_casts(q, mode, sort_type, author_fid, viewer_fid, parent_url, channel_id, priority_mode, limit, cursor, x_neynar_experimental)
Search for casts

Search for casts based on a query string, with optional AND filters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Query string to search for casts. Supported operators:  | Operator  | Description                                                                                              | | --------- | -------------------------------------------------------------------------------------------------------- | | `+`       | Acts as the AND operator. This is the default operator between terms and can usually be omitted.         | | `\\|`      | Acts as the OR operator.                                                                                 | | `*`       | When used at the end of a term, signifies a prefix query.                                                  | | `\"`       | Wraps several terms into a phrase (for example, `\"star wars\"`).                                          | | `(`, `)`  | Wrap a clause for precedence (for example, `star + (wars \\| trek)`).                                     | | `~n`      | When used after a term (for example, `satr~3`), sets `fuzziness`. When used after a phrase, sets `slop`. | | `-`       | Negates the term.                                                                                        | | `before:` | Search for casts before a specific date. (e.g. `before:2025-04-20`)                                       | | `after:`  | Search for casts after a specific date. (e.g. `after:2025-04-20`)                                         |  | [required] |
**mode** | Option<**String**> | Choices are: - `literal` - Searches for the words in the query string (default) - `semantic` - Searches for the meaning of the query string - `hybrid` - Combines both literal and semantic results  |  |
**sort_type** | Option<[**SearchSortType**](.md)> | Choices are: - `desc_chron` - All casts sorted by time (default) - `algorithmic` - Casts sorted by engagement and time  |  |
**author_fid** | Option<**i32**> | Fid of the user whose casts you want to search |  |
**viewer_fid** | Option<**i32**> | Providing this will return search results that respects this user's mutes and blocks and includes `viewer_context`. |  |
**parent_url** | Option<**String**> | Parent URL of the casts you want to search |  |
**channel_id** | Option<**String**> | Channel ID of the casts you want to search |  |
**priority_mode** | Option<**bool**> | When true, only returns search results from power badge users and users that the viewer follows (if viewer_fid is provided). |  |[default to false]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::CastsSearchResponse**](CastsSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

