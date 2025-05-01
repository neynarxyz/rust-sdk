# \UserDataApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_user_data**](UserDataApi.md#fetch_user_data) | **GET** /v1/userDataByFid | Fetch UserData for a FID



## fetch_user_data

> models::FetchUserData200Response fetch_user_data(fid, user_data_type, page_size, reverse, page_token)
Fetch UserData for a FID

**Note:** one of two different response schemas is returned based on whether the caller provides the `user_data_type` parameter. If included, a single `UserDataAdd` message is returned (or a `not_found` error). If omitted, a paginated list of `UserDataAdd` messages is returned instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID that's being requested | [required] |
**user_data_type** | Option<[**UserDataType**](.md)> | The type of user data, either as a numerical value or type string. If this is omitted, all user data for the FID is returned |  |
**page_size** | Option<**i32**> | Maximum number of messages to return in a single response |  |
**reverse** | Option<**bool**> | Reverse the sort order, returning latest messages first |  |
**page_token** | Option<**String**> | The page token returned by the previous query, to fetch the next page. If this parameter is empty, fetch the first page |  |

### Return type

[**models::FetchUserData200Response**](fetch_user_data_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

