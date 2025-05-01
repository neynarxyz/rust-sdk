# \ReactionApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_reaction**](ReactionApi.md#delete_reaction) | **DELETE** /farcaster/reaction | Delete reaction
[**fetch_cast_reactions**](ReactionApi.md#fetch_cast_reactions) | **GET** /farcaster/reactions/cast | Reactions for cast
[**fetch_user_reactions**](ReactionApi.md#fetch_user_reactions) | **GET** /farcaster/reactions/user | Reactions for user
[**publish_reaction**](ReactionApi.md#publish_reaction) | **POST** /farcaster/reaction | Post a reaction



## delete_reaction

> models::OperationResponse delete_reaction(reaction_req_body)
Delete reaction

Delete a reaction (like or recast) to a cast \\ (In order to delete a reaction `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reaction_req_body** | [**ReactionReqBody**](ReactionReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_cast_reactions

> models::ReactionsCastResponse fetch_cast_reactions(hash, types, viewer_fid, limit, cursor)
Reactions for cast

Fetches reactions for a given cast

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |[default to 0xfe90f9de682273e05b201629ad2338bdcd89b6be]
**types** | [**Vec<models::ReactionsType>**](models::ReactionsType.md) | Customize which reaction types the request should search for. This is a comma-separated string that can include the following values: 'likes' and 'recasts'. By default api returns both. To select multiple types, use a comma-separated list of these values.  | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a list of reactions that respects this user's mutes and blocks and includes `viewer_context`. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ReactionsCastResponse**](ReactionsCastResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_reactions

> models::ReactionsResponse fetch_user_reactions(fid, r#type, viewer_fid, limit, cursor)
Reactions for user

Fetches reactions for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |
**r#type** | [**ReactionsType**](.md) | Type of reaction to fetch (likes or recasts or all) | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a list of reactions that respects this user's mutes and blocks and includes `viewer_context`. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ReactionsResponse**](ReactionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_reaction

> models::OperationResponse publish_reaction(reaction_req_body)
Post a reaction

Post a reaction (like or recast) to a given cast \\ (In order to post a reaction `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reaction_req_body** | [**ReactionReqBody**](ReactionReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

