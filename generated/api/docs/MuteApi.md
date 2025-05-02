# \MuteApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_mute**](MuteApi.md#delete_mute) | **DELETE** /farcaster/mute | Unmute FID
[**fetch_mute_list**](MuteApi.md#fetch_mute_list) | **GET** /farcaster/mute/list | Muted FIDs of user
[**publish_mute**](MuteApi.md#publish_mute) | **POST** /farcaster/mute | Mute FID



## delete_mute

> models::MuteResponse delete_mute(mute_req_body)
Unmute FID

Deletes a mute for a given FID. This is an allowlisted API, reach out if you want access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mute_req_body** | [**MuteReqBody**](MuteReqBody.md) |  | [required] |

### Return type

[**models::MuteResponse**](MuteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_mute_list

> models::MuteListResponse fetch_mute_list(fid, limit, cursor, x_neynar_experimental)
Muted FIDs of user

Fetches all FIDs that a user has muted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The user's FID (identifier) | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::MuteListResponse**](MuteListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_mute

> models::MuteResponse publish_mute(mute_req_body)
Mute FID

Adds a mute for a given FID. This is an allowlisted API, reach out if you want access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mute_req_body** | [**MuteReqBody**](MuteReqBody.md) |  | [required] |

### Return type

[**models::MuteResponse**](MuteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

