# \VerificationsApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_verifications_by_fid**](VerificationsApi.md#fetch_verifications_by_fid) | **GET** /v1/verificationsByFid | Provided by an FID



## fetch_verifications_by_fid

> models::FetchVerificationsByFid200Response fetch_verifications_by_fid(fid, address, page_size, reverse, page_token)
Provided by an FID

Fetch verifications provided by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID being requested | [required] |
**address** | Option<**String**> | The optional ETH address to filter by |  |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchVerificationsByFid200Response**](fetch_verifications_by_fid_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

