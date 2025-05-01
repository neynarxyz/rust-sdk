# \HubEventsApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_events**](HubEventsApi.md#fetch_events) | **GET** /v1/events | Page of events
[**lookup_event**](HubEventsApi.md#lookup_event) | **GET** /v1/eventById | Event by ID



## fetch_events

> models::FetchEvents200Response fetch_events(from_event_id)
Page of events

Fetch a list of events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_event_id** | Option<**i32**> | An optional Hub Id to start getting events from. This is also returned from the API as nextPageEventId, which can be used to page through all the Hub events. Set it to 0 to start from the first event.  |  |

### Return type

[**models::FetchEvents200Response**](fetch_events_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_event

> models::HubEvent lookup_event(event_id)
Event by ID

Lookup an event by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **i32** | The Hub Id of the event | [required] |

### Return type

[**models::HubEvent**](HubEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

