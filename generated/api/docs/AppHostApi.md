# \AppHostApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_host_get_event**](AppHostApi.md#app_host_get_event) | **GET** /farcaster/app_host/user/event | Get app host event
[**app_host_get_user_state**](AppHostApi.md#app_host_get_user_state) | **GET** /farcaster/app_host/user/state | Get the user's notification subscriptions
[**app_host_post_event**](AppHostApi.md#app_host_post_event) | **POST** /farcaster/app_host/user/event | Process app host event



## app_host_get_event

> models::AppHostGetEventResponse app_host_get_event(app_domain, fid, event)
Get app host event

Returns event object for app host events. Used if the app host intends to sign the event message instead of using Neynar-hosted signers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_domain** | **String** | The domain of the mini app | [required] |
**fid** | **i32** | The FID of the user who initiated the event | [required] |
**event** | [**AppHostEventType**](.md) | The type of event | [required] |

### Return type

[**models::AppHostGetEventResponse**](AppHostGetEventResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_host_get_user_state

> models::AppHostUserStateResponse app_host_get_user_state(fid)
Get the user's notification subscriptions

Returns the current notification state for a specific user across all mini app domains in this app host. Shows which domains have notifications enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the user | [required] |

### Return type

[**models::AppHostUserStateResponse**](AppHostUserStateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_host_post_event

> models::AppHostPostEventResponse app_host_post_event(app_host_post_event_body)
Process app host event

Post an app_host event to the domain's webhook. Events such as enabling or disabling notifications for a user. Provide either a signed message or the signer UUID of an authorized neynar-hosted signers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_host_post_event_body** | [**AppHostPostEventBody**](AppHostPostEventBody.md) |  | [required] |

### Return type

[**models::AppHostPostEventResponse**](AppHostPostEventResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

