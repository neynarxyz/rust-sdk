# \CastsApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_casts_by_parent**](CastsApi.md#fetch_casts_by_parent) | **GET** /v1/castsByParent | By parent cast
[**fetch_casts_mentioning_user**](CastsApi.md#fetch_casts_mentioning_user) | **GET** /v1/castsByMention | Mentioning an FID
[**fetch_users_casts**](CastsApi.md#fetch_users_casts) | **GET** /v1/castsByFid | By FID
[**lookup_cast_by_hash_and_fid**](CastsApi.md#lookup_cast_by_hash_and_fid) | **GET** /v1/castById | By FID and Hash



## fetch_casts_by_parent

> models::FetchCastsByParent200Response fetch_casts_by_parent(fid, hash, url, page_size, reverse, page_token)
By parent cast

Retrieve all reply casts (responses) to a specific parent cast in the Farcaster network. Parent casts can be identified using either a combination of FID and hash, or by their URL. This endpoint enables traversal of conversation threads and retrieval of all responses to a particular cast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | Option<**i32**> | The Farcaster ID (FID) of the parent cast's creator. This parameter must be used together with the 'hash' parameter to uniquely identify a parent cast. Required only when using hash-based lookup instead of URL-based lookup. The FID is a unique identifier assigned to each Farcaster user. |  |
**hash** | Option<**String**> | The unique hash identifier of the parent cast. Must be used together with the 'fid' parameter when doing hash-based lookup. This is a 40-character hexadecimal string prefixed with '0x' that uniquely identifies the cast within the creator's posts. Not required if using URL-based lookup. |  |
**url** | Option<**String**> | Cast URL starting with 'chain://' |  |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchCastsByParent200Response**](fetch_casts_by_parent_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_casts_mentioning_user

> models::FetchCastsMentioningUser200Response fetch_casts_mentioning_user(fid, page_size, reverse, page_token)
Mentioning an FID

Fetch casts mentioning a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID that is mentioned in a cast | [required] |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchCastsMentioningUser200Response**](fetch_casts_mentioning_user_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_casts

> models::FetchUsersCasts200Response fetch_users_casts(fid, page_size, reverse, page_token)
By FID

Fetch user's casts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the casts' creator | [required] |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchUsersCasts200Response**](fetch_users_casts_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_cast_by_hash_and_fid

> models::CastAdd lookup_cast_by_hash_and_fid(fid, hash)
By FID and Hash

Lookup a cast by its FID and hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the cast's creator | [required] |
**hash** | **String** | The unique hash identifier of the cast. This is a 40-character hexadecimal string prefixed with '0x' that uniquely identifies a specific cast in the Farcaster network. | [required] |

### Return type

[**models::CastAdd**](CastAdd.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

