# \MetricsApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_cast_metrics**](MetricsApi.md#fetch_cast_metrics) | **GET** /farcaster/cast/metrics | Metrics for casts



## fetch_cast_metrics

> models::CastsMetricsResponse fetch_cast_metrics(q, interval, author_fid, channel_id, x_neynar_experimental)
Metrics for casts

Fetches metrics casts matching a query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Query string to search for casts | [required] |
**interval** | Option<**String**> | Interval of time for which to fetch metrics. Default is 30d. |  |
**author_fid** | Option<**i32**> | Fid of the user whose casts you want to search |  |
**channel_id** | Option<**String**> | Channel ID of the casts you want to search |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::CastsMetricsResponse**](CastsMetricsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

