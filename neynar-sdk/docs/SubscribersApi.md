# \SubscribersApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_subscribed_to_for_fid**](SubscribersApi.md#fetch_subscribed_to_for_fid) | **GET** /farcaster/user/subscribed_to | Subscribed to
[**fetch_subscribers_for_fid**](SubscribersApi.md#fetch_subscribers_for_fid) | **GET** /farcaster/user/subscribers | Subscribers of a user
[**fetch_subscription_check**](SubscribersApi.md#fetch_subscription_check) | **GET** /stp/subscription_check | Hypersub subscription check
[**fetch_subscriptions_for_fid**](SubscribersApi.md#fetch_subscriptions_for_fid) | **GET** /farcaster/user/subscriptions_created | Subscriptions created by FID



## fetch_subscribed_to_for_fid

> models::SubscribedToResponse fetch_subscribed_to_for_fid(fid, subscription_provider, viewer_fid)
Subscribed to

Fetch what FIDs and contracts a FID is subscribed to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |
**subscription_provider** | [**SubscriptionProvider**](.md) |  | [required] |
**viewer_fid** | Option<**i32**> |  |  |

### Return type

[**models::SubscribedToResponse**](SubscribedToResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_subscribers_for_fid

> models::SubscribersResponse fetch_subscribers_for_fid(fid, subscription_provider, viewer_fid)
Subscribers of a user

Fetch subscribers for a given FID's contracts. Doesn't return addresses that don't have an FID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |
**subscription_provider** | [**SubscriptionProviders**](.md) |  | [required] |
**viewer_fid** | Option<**i32**> |  |  |

### Return type

[**models::SubscribersResponse**](SubscribersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_subscription_check

> std::collections::HashMap<String, models::SubscriptionStatus> fetch_subscription_check(addresses, contract_address, chain_id)
Hypersub subscription check

Check if a wallet address is subscribed to a given STP (Hypersub) contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addresses** | **String** | Comma separated list of Ethereum addresses, up to 350 at a time | [required] |
**contract_address** | **String** | Ethereum address of the STP contract | [required] |
**chain_id** | **String** | Chain ID of the STP contract | [required] |

### Return type

[**std::collections::HashMap<String, models::SubscriptionStatus>**](SubscriptionStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_subscriptions_for_fid

> models::SubscriptionsResponse fetch_subscriptions_for_fid(fid, subscription_provider)
Subscriptions created by FID

Fetch created subscriptions for a given FID's.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** |  | [required] |
**subscription_provider** | [**SubscriptionProvider**](.md) |  | [required] |

### Return type

[**models::SubscriptionsResponse**](SubscriptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

