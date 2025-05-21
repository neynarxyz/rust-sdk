# \FrameApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_neynar_frame**](FrameApi.md#delete_neynar_frame) | **DELETE** /farcaster/frame | Delete mini app
[**fetch_frame_catalog**](FrameApi.md#fetch_frame_catalog) | **GET** /farcaster/frame/catalog | Mini apps catalog
[**fetch_frame_meta_tags_from_url**](FrameApi.md#fetch_frame_meta_tags_from_url) | **GET** /farcaster/frame/crawl | Meta tags from URL
[**fetch_neynar_frames**](FrameApi.md#fetch_neynar_frames) | **GET** /farcaster/frame/list | List of mini apps
[**fetch_notification_tokens**](FrameApi.md#fetch_notification_tokens) | **GET** /farcaster/frame/notification_tokens | List of mini app notification tokens 
[**fetch_relevant_frames**](FrameApi.md#fetch_relevant_frames) | **GET** /farcaster/frame/relevant | Relevant mini apps
[**fetch_validate_frame_analytics**](FrameApi.md#fetch_validate_frame_analytics) | **GET** /farcaster/frame/validate/analytics | Analytics for the mini app
[**fetch_validate_frame_list**](FrameApi.md#fetch_validate_frame_list) | **GET** /farcaster/frame/validate/list | All mini apps validated by user
[**get_transaction_pay_frame**](FrameApi.md#get_transaction_pay_frame) | **GET** /farcaster/frame/transaction/pay | Get transaction pay mini app
[**lookup_neynar_frame**](FrameApi.md#lookup_neynar_frame) | **GET** /farcaster/frame | Mini app by UUID or URL
[**post_frame_action**](FrameApi.md#post_frame_action) | **POST** /farcaster/frame/action | Post a mini app action, cast action or a cast composer action
[**post_frame_action_developer_managed**](FrameApi.md#post_frame_action_developer_managed) | **POST** /farcaster/frame/developer_managed/action | Signature packet
[**publish_frame_notifications**](FrameApi.md#publish_frame_notifications) | **POST** /farcaster/frame/notifications | Send notifications
[**publish_neynar_frame**](FrameApi.md#publish_neynar_frame) | **POST** /farcaster/frame | Create mini app
[**search_frames**](FrameApi.md#search_frames) | **GET** /farcaster/frame/search | Search mini apps
[**update_neynar_frame**](FrameApi.md#update_neynar_frame) | **PUT** /farcaster/frame | Update mini app
[**validate_frame_action**](FrameApi.md#validate_frame_action) | **POST** /farcaster/frame/validate | Validate mini app action



## delete_neynar_frame

> models::DeleteFrameResponse delete_neynar_frame(delete_frame_req_body)
Delete mini app

Delete an existing mini app, if it was made by the developer (identified by API key)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_frame_req_body** | [**DeleteFrameReqBody**](DeleteFrameReqBody.md) |  | [required] |

### Return type

[**models::DeleteFrameResponse**](DeleteFrameResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_frame_catalog

> models::FrameCatalogResponse fetch_frame_catalog(limit, cursor, time_window, categories)
Mini apps catalog

A curated list of featured mini apps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor |  |
**time_window** | Option<[**MiniAppTimeWindow**](.md)> | Time window used to calculate the change in trending score for each mini app, used to sort mini app results |  |
**categories** | Option<[**Vec<String>**](String.md)> | Comma separated list of categories to include in the results.  Includes all if left blank.  Example: `categories=games,social` OR: `categories=games&categories=social` |  |

### Return type

[**models::FrameCatalogResponse**](FrameCatalogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_frame_meta_tags_from_url

> models::FetchFrameMetaTagsFromUrl200Response fetch_frame_meta_tags_from_url(url)
Meta tags from URL

Fetches the mini app meta tags from the URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | The mini app URL to crawl | [required] |

### Return type

[**models::FetchFrameMetaTagsFromUrl200Response**](fetch_frame_meta_tags_from_url_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_neynar_frames

> Vec<models::NeynarFrame> fetch_neynar_frames()
List of mini apps

Fetch a list of mini apps made by the developer (identified by API key)

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NeynarFrame>**](NeynarFrame.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_notification_tokens

> models::FrameNotificationTokens fetch_notification_tokens(limit, fids, cursor)
List of mini app notification tokens 

Returns a list of notifications tokens related to a mini app 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**fids** | Option<**String**> | Comma separated list of FIDs, up to 100 at a time. If you pass in FIDs, you will get back the notification tokens for those FIDs. If you don't pass in FIDs, you will get back all the notification tokens for the mini app. |  |
**cursor** | Option<**String**> | Pagination cursor |  |

### Return type

[**models::FrameNotificationTokens**](FrameNotificationTokens.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_relevant_frames

> models::FetchRelevantFrames200Response fetch_relevant_frames(viewer_fid, time_window)
Relevant mini apps

Fetch a list of mini apps relevant to the user based on casts by users with strong affinity score for the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**viewer_fid** | **i32** | FID of the user to fetch relevant mini apps for | [required] |
**time_window** | Option<[**MiniAppTimeWindow**](.md)> | Time window used to limit statistics used to calculate mini app relevance |  |

### Return type

[**models::FetchRelevantFrames200Response**](fetch_relevant_frames_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_validate_frame_analytics

> models::FrameValidateAnalyticsResponse fetch_validate_frame_analytics(frame_url, analytics_type, start, stop, aggregate_window)
Analytics for the mini app

Fetch analytics for total-interactors, interactors, nteractions-per-cast and input-text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frame_url** | **String** |  | [required] |
**analytics_type** | [**ValidateFrameAnalyticsType**](.md) |  | [required] |
**start** | **String** |  | [required] |[default to 2024-04-06T06:44:56.811Z]
**stop** | **String** |  | [required] |[default to 2024-04-08T06:44:56.811Z]
**aggregate_window** | Option<[**ValidateFrameAggregateWindow**](.md)> | Required for `analytics_type=interactions-per-cast` |  |

### Return type

[**models::FrameValidateAnalyticsResponse**](FrameValidateAnalyticsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_validate_frame_list

> models::FrameValidateListResponse fetch_validate_frame_list()
All mini apps validated by user

Fetch a list of all the mini apps validated by a user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FrameValidateListResponse**](FrameValidateListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_pay_frame

> models::TransactionFrameResponse get_transaction_pay_frame(id)
Get transaction pay mini app

Retrieves details about a transaction pay mini app by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the transaction mini app to retrieve | [required] |

### Return type

[**models::TransactionFrameResponse**](TransactionFrameResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_neynar_frame

> models::NeynarFrame lookup_neynar_frame(r#type, uuid, url)
Mini app by UUID or URL

Fetch a mini app either by UUID or Neynar URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | [**FrameType**](.md) |  | [required] |
**uuid** | Option<**uuid::Uuid**> | UUID of the mini app to fetch |  |
**url** | Option<**String**> | URL of the Neynar mini app to fetch |  |

### Return type

[**models::NeynarFrame**](NeynarFrame.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_frame_action

> models::Frame post_frame_action(frame_action_req_body)
Post a mini app action, cast action or a cast composer action

Post mini app actions, cast actions or cast composer actions to the server  \\ (In order to post any of these actions, you need to have an approved `signer_uuid`)  The POST request to the post_url has a timeout of 5 seconds for mini apps. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frame_action_req_body** | [**FrameActionReqBody**](FrameActionReqBody.md) |  | [required] |

### Return type

[**models::Frame**](Frame.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_frame_action_developer_managed

> models::Frame post_frame_action_developer_managed(frame_developer_managed_action_req_body)
Signature packet

Post a mini app action that has been signed with a developer managed signer  The POST request to the post_url has a timeout of 5 seconds. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frame_developer_managed_action_req_body** | [**FrameDeveloperManagedActionReqBody**](FrameDeveloperManagedActionReqBody.md) |  | [required] |

### Return type

[**models::Frame**](Frame.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_frame_notifications

> models::SendFrameNotificationsResponse publish_frame_notifications(send_frame_notifications_req_body)
Send notifications

Send notifications to interactors of a mini app 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_frame_notifications_req_body** | [**SendFrameNotificationsReqBody**](SendFrameNotificationsReqBody.md) |  | [required] |

### Return type

[**models::SendFrameNotificationsResponse**](SendFrameNotificationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_neynar_frame

> models::NeynarFrame publish_neynar_frame(neynar_frame_creation_req_body)
Create mini app

Create a new mini app with a list of pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**neynar_frame_creation_req_body** | [**NeynarFrameCreationReqBody**](NeynarFrameCreationReqBody.md) |  | [required] |

### Return type

[**models::NeynarFrame**](NeynarFrame.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_frames

> models::FrameCatalogResponse search_frames(q, limit, cursor)
Search mini apps

Search for mini apps based on a query string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Query string to search for mini apps | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor |  |

### Return type

[**models::FrameCatalogResponse**](FrameCatalogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_neynar_frame

> models::NeynarFrame update_neynar_frame(neynar_frame_update_req_body)
Update mini app

Update an existing mini app with a list of pages, if it was made by the developer (identified by API key)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**neynar_frame_update_req_body** | [**NeynarFrameUpdateReqBody**](NeynarFrameUpdateReqBody.md) |  | [required] |

### Return type

[**models::NeynarFrame**](NeynarFrame.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_frame_action

> models::ValidateFrameActionResponse validate_frame_action(validate_frame_action_req_body)
Validate mini app action

Validates a mini app against by an interacting user against a Farcaster Hub \\ (In order to validate a mini app, message bytes from Frame Action must be provided in hex) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_frame_action_req_body** | [**ValidateFrameActionReqBody**](ValidateFrameActionReqBody.md) |  | [required] |

### Return type

[**models::ValidateFrameActionResponse**](ValidateFrameActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

