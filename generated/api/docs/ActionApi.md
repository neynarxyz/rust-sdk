# \ActionApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**publish_farcaster_action**](ActionApi.md#publish_farcaster_action) | **POST** /farcaster/action | User actions across apps



## publish_farcaster_action

> std::collections::HashMap<String, serde_json::Value> publish_farcaster_action(farcaster_action_req_body)
User actions across apps

Securely communicate and perform actions on behalf of users across different apps. It enables an app to send data or trigger actions in another app on behalf of a mutual user by signing messages using the user's Farcaster signer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**farcaster_action_req_body** | [**FarcasterActionReqBody**](FarcasterActionReqBody.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

