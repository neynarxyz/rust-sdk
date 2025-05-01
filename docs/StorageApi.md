# \StorageApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**buy_storage**](StorageApi.md#buy_storage) | **POST** /farcaster/storage/buy | Buy storage
[**lookup_user_storage_allocations**](StorageApi.md#lookup_user_storage_allocations) | **GET** /farcaster/storage/allocations | Allocation of user
[**lookup_user_storage_usage**](StorageApi.md#lookup_user_storage_usage) | **GET** /farcaster/storage/usage | Usage of user



## buy_storage

> models::StorageAllocationsResponse buy_storage(buy_storage_req_body)
Buy storage

This api will help you rent units of storage for an year for a specific FID. A storage unit lets you store 5000 casts, 2500 reactions and 2500 links. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buy_storage_req_body** | [**BuyStorageReqBody**](BuyStorageReqBody.md) |  | [required] |

### Return type

[**models::StorageAllocationsResponse**](StorageAllocationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_user_storage_allocations

> models::StorageAllocationsResponse lookup_user_storage_allocations(fid)
Allocation of user

Fetches storage allocations for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |

### Return type

[**models::StorageAllocationsResponse**](StorageAllocationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_user_storage_usage

> models::StorageUsageResponse lookup_user_storage_usage(fid)
Usage of user

Fetches storage usage for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |

### Return type

[**models::StorageUsageResponse**](StorageUsageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

