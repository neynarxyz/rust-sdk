# \StorageApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lookup_user_storage_limit**](StorageApi.md#lookup_user_storage_limit) | **GET** /v1/storageLimitsByFid | FID's limits



## lookup_user_storage_limit

> models::StorageLimitsResponse lookup_user_storage_limit(fid)
FID's limits

Fetch a user's storage limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |

### Return type

[**models::StorageLimitsResponse**](StorageLimitsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

