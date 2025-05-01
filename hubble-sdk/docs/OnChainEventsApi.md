# \OnChainEventsApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_user_on_chain_events**](OnChainEventsApi.md#fetch_user_on_chain_events) | **GET** /v1/onChainEventsByFid | Fetch a list of on-chain events provided by an FID
[**fetch_user_on_chain_signers_events**](OnChainEventsApi.md#fetch_user_on_chain_signers_events) | **GET** /v1/onChainSignersByFid | Fetch a list of signers provided by an FID
[**lookup_on_chain_id_registry_event_by_address**](OnChainEventsApi.md#lookup_on_chain_id_registry_event_by_address) | **GET** /v1/onChainIdRegistryEventByAddress | Fetch an on-chain ID Registry Event for a given Address



## fetch_user_on_chain_events

> models::FetchUserOnChainEvents200Response fetch_user_on_chain_events(fid, event_type)
Fetch a list of on-chain events provided by an FID

Fetch on-chain events provided by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID being requested | [required] |
**event_type** | [**OnChainEventType**](.md) | The numeric or string value of the event type being requested | [required] |

### Return type

[**models::FetchUserOnChainEvents200Response**](fetch_user_on_chain_events_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_on_chain_signers_events

> models::FetchUserOnChainSignersEvents200Response fetch_user_on_chain_signers_events(fid, signer)
Fetch a list of signers provided by an FID

**Note:** one of two different response schemas is returned based on whether the caller provides the `signer` parameter. If included, a single `OnChainEventSigner` message is returned (or a `not_found` error). If omitted, a non-paginated list of `OnChainEventSigner` messages is returned instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID being requested | [required] |
**signer** | Option<**String**> | The optional key of signer |  |

### Return type

[**models::FetchUserOnChainSignersEvents200Response**](fetch_user_on_chain_signers_events_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_on_chain_id_registry_event_by_address

> models::OnChainEventIdRegister lookup_on_chain_id_registry_event_by_address(address)
Fetch an on-chain ID Registry Event for a given Address

Fetch an on-chain ID Registry Event for a given Address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The ETH address being requested | [required] |

### Return type

[**models::OnChainEventIdRegister**](OnChainEventIdRegister.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

