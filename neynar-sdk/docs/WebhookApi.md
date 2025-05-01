# \WebhookApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webhook**](WebhookApi.md#delete_webhook) | **DELETE** /farcaster/webhook | Delete a webhook
[**fetch_webhooks**](WebhookApi.md#fetch_webhooks) | **GET** /farcaster/webhook/list | Associated webhooks of user
[**lookup_webhook**](WebhookApi.md#lookup_webhook) | **GET** /farcaster/webhook | Fetch a webhook
[**publish_webhook**](WebhookApi.md#publish_webhook) | **POST** /farcaster/webhook | Create a webhook
[**update_webhook**](WebhookApi.md#update_webhook) | **PUT** /farcaster/webhook | Update a webhook
[**update_webhook_active_status**](WebhookApi.md#update_webhook_active_status) | **PATCH** /farcaster/webhook | Update webhook status



## delete_webhook

> models::WebhookResponse delete_webhook(webhook_delete_req_body)
Delete a webhook

Delete a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_delete_req_body** | [**WebhookDeleteReqBody**](WebhookDeleteReqBody.md) |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_webhooks

> models::WebhookListResponse fetch_webhooks()
Associated webhooks of user

Fetch a list of webhooks associated to a user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookListResponse**](WebhookListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_webhook

> models::WebhookResponse lookup_webhook(webhook_id)
Fetch a webhook

Fetch a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_webhook

> models::WebhookResponse publish_webhook(webhook_post_req_body)
Create a webhook

Create a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_post_req_body** | [**WebhookPostReqBody**](WebhookPostReqBody.md) |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::WebhookResponse update_webhook(webhook_put_req_body)
Update a webhook

Update a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_put_req_body** | [**WebhookPutReqBody**](WebhookPutReqBody.md) |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_active_status

> models::WebhookResponse update_webhook_active_status(webhook_patch_req_body)
Update webhook status

Update webhook active status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_patch_req_body** | [**WebhookPatchReqBody**](WebhookPatchReqBody.md) |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

