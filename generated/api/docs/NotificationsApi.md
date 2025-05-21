# \NotificationsApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_all_notifications**](NotificationsApi.md#fetch_all_notifications) | **GET** /farcaster/notifications | For user
[**fetch_channel_notifications_for_user**](NotificationsApi.md#fetch_channel_notifications_for_user) | **GET** /farcaster/notifications/channel | For user by channel
[**fetch_notifications_by_parent_url_for_user**](NotificationsApi.md#fetch_notifications_by_parent_url_for_user) | **GET** /farcaster/notifications/parent_url | For user by parent_urls
[**mark_notifications_as_seen**](NotificationsApi.md#mark_notifications_as_seen) | **POST** /farcaster/notifications/seen | Mark as seen



## fetch_all_notifications

> models::NotificationsResponse fetch_all_notifications(fid, r#type, priority_mode, limit, cursor)
For user

Returns a list of notifications for a specific FID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of the user you you want to fetch notifications for. The response will respect this user's mutes and blocks. | [required] |
**r#type** | Option<[**Vec<models::NotificationType>**](models::NotificationType.md)> | Notification type to fetch. Comma separated values of follows, recasts, likes, mentions, replies. |  |
**priority_mode** | Option<**bool**> | When true, only returns notifications from power badge users and users that the user follows. |  |[default to false]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 15]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::NotificationsResponse**](NotificationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_channel_notifications_for_user

> models::NotificationsResponse fetch_channel_notifications_for_user(fid, channel_ids, priority_mode, limit, cursor)
For user by channel

Returns a list of notifications for a user in specific channels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of the user you you want to fetch notifications for. The response will respect this user's mutes and blocks. | [required] |
**channel_ids** | **String** | Comma separated channel_ids (find list of all channels here - https://docs.neynar.com/reference/list-all-channels) | [required] |
**priority_mode** | Option<**bool**> | When true, only returns notifications from power badge users and users that the user follows. |  |[default to false]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 15]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::NotificationsResponse**](NotificationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_notifications_by_parent_url_for_user

> models::NotificationsResponse fetch_notifications_by_parent_url_for_user(fid, parent_urls, priority_mode, limit, cursor)
For user by parent_urls

Returns a list of notifications for a user in specific parent_urls

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of the user you you want to fetch notifications for. The response will respect this user's mutes and blocks. | [required] |
**parent_urls** | **String** | Comma separated parent_urls | [required] |
**priority_mode** | Option<**bool**> | When true, only returns notifications from power badge users and users that the user follows. |  |[default to false]
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 15]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::NotificationsResponse**](NotificationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_notifications_as_seen

> models::OperationResponse mark_notifications_as_seen(mark_notifications_as_seen_req_body, authorization)
Mark as seen

Mark notifications as seen. You can choose one of two authorization methods, either:   1. Provide a valid signer_uuid in the request body (Most common)   2. Provide a valid, signed \"Bearer\" token in the request's `Authorization` header similar to the      approach described [here](https://docs.farcaster.xyz/reference/warpcast/api#authentication) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mark_notifications_as_seen_req_body** | [**MarkNotificationsAsSeenReqBody**](MarkNotificationsAsSeenReqBody.md) |  | [required] |
**authorization** | Option<**String**> | Optional Bearer token for certain endpoints. The token format is described [here](https://docs.farcaster.xyz/reference/warpcast/api#authentication).  |  |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

