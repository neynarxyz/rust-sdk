# \MetricsApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_cast_metrics**](MetricsApi.md#fetch_cast_metrics) | **GET** /farcaster/cast/metrics | Metrics for casts



## fetch_cast_metrics

> models::CastsMetricsResponse fetch_cast_metrics(q, interval, author_fid, channel_id)
Metrics for casts

Fetches metrics casts matching a query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Query string to search for casts | [required] |
**interval** | Option<**String**> | Interval of time for which to fetch metrics. Choices are `1d`, `7d`, `30d` |  |
**author_fid** | Option<**i32**> | Fid of the user whose casts you want to search |  |
**channel_id** | Option<**String**> | Channel ID of the casts you want to search |  |

### Return type

[**models::CastsMetricsResponse**](CastsMetricsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

