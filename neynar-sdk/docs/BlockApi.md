# \BlockApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_block**](BlockApi.md#delete_block) | **DELETE** /farcaster/block | Unblock FID
[**fetch_block_list**](BlockApi.md#fetch_block_list) | **GET** /farcaster/block/list | Blocked / Blocked by FIDs
[**publish_block**](BlockApi.md#publish_block) | **POST** /farcaster/block | Block FID



## delete_block

> models::OperationResponse delete_block(block_req_body)
Unblock FID

Deletes a block for a given FID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_req_body** | [**BlockReqBody**](BlockReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_block_list

> models::BlockListResponse fetch_block_list(blocker_fid, blocked_fid, limit, cursor, x_neynar_experimental)
Blocked / Blocked by FIDs

Fetches all FIDs that a user has blocked or has been blocked by

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blocker_fid** | Option<**i32**> | Providing this will return the users that this user has blocked |  |
**blocked_fid** | Option<**i32**> | Providing this will return the users that have blocked this user |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::BlockListResponse**](BlockListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_block

> models::OperationResponse publish_block(block_req_body)
Block FID

Adds a block for a given FID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_req_body** | [**BlockReqBody**](BlockReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

