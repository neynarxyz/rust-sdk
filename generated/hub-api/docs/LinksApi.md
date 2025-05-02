# \LinksApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_user_followers**](LinksApi.md#fetch_user_followers) | **GET** /v1/linksByTargetFid | To target FID
[**fetch_user_following**](LinksApi.md#fetch_user_following) | **GET** /v1/linksByFid | From source FID
[**lookup_user_relation**](LinksApi.md#lookup_user_relation) | **GET** /v1/linkById | By its FID and target FID



## fetch_user_followers

> models::FetchUserFollowers200Response fetch_user_followers(target_fid, link_type, page_size, reverse, page_token)
To target FID

Fetch a list of users that are following a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_fid** | **i32** | The FID of the target user for this link | [required] |
**link_type** | Option<[**LinkType**](.md)> |  |  |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchUserFollowers200Response**](fetch_user_followers_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_following

> models::FetchUserFollowing200Response fetch_user_following(fid, link_type, page_size, reverse, page_token)
From source FID

Fetch a list of users that a user is following.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the link's originator | [required] |
**link_type** | Option<[**LinkType**](.md)> |  |  |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchUserFollowing200Response**](fetch_user_following_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_user_relation

> models::LinkAdd lookup_user_relation(fid, target_fid, link_type)
By its FID and target FID

Lookup a link by its FID and target FID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the link's originator | [required] |
**target_fid** | **i32** | The FID of the target user for this link | [required] |
**link_type** | [**LinkType**](.md) |  | [required] |

### Return type

[**models::LinkAdd**](LinkAdd.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

