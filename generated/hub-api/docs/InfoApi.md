# \InfoApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lookup_hub_info**](InfoApi.md#lookup_hub_info) | **GET** /v1/info | Sync Methods



## lookup_hub_info

> models::HubInfoResponse lookup_hub_info(dbstats)
Sync Methods

Retrieve hub information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbstats** | **bool** | Controls whether the response includes database statistics. When true, the response includes information about the hub's database state, storage usage, and performance metrics. | [required] |

### Return type

[**models::HubInfoResponse**](HubInfoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

