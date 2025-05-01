# \BanApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_bans**](BanApi.md#delete_bans) | **DELETE** /farcaster/ban | Unban FIDs from app
[**fetch_ban_list**](BanApi.md#fetch_ban_list) | **GET** /farcaster/ban/list | Banned FIDs of app
[**publish_bans**](BanApi.md#publish_bans) | **POST** /farcaster/ban | Ban FIDs from app



## delete_bans

> models::BanResponse delete_bans(ban_req_body)
Unban FIDs from app

Deletes a list of FIDs from the app associated with your API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_req_body** | [**BanReqBody**](BanReqBody.md) |  | [required] |

### Return type

[**models::BanResponse**](BanResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_ban_list

> models::BanListResponse fetch_ban_list(limit, cursor, x_neynar_experimental)
Banned FIDs of app

Fetches all FIDs that your app has banned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::BanListResponse**](BanListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_bans

> models::BanResponse publish_bans(ban_req_body)
Ban FIDs from app

Bans a list of FIDs from the app associated with your API key. Banned users, their casts and reactions will not appear in feeds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_req_body** | [**BanReqBody**](BanReqBody.md) |  | [required] |

### Return type

[**models::BanResponse**](BanResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

