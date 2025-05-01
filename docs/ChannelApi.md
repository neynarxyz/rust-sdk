# \ChannelApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_all_channels**](ChannelApi.md#fetch_all_channels) | **GET** /farcaster/channel/list | Fetch all channels with their details
[**fetch_bulk_channels**](ChannelApi.md#fetch_bulk_channels) | **GET** /farcaster/channel/bulk | Bulk fetch
[**fetch_channel_invites**](ChannelApi.md#fetch_channel_invites) | **GET** /farcaster/channel/member/invite/list | Open invites
[**fetch_channel_members**](ChannelApi.md#fetch_channel_members) | **GET** /farcaster/channel/member/list | Fetch members
[**fetch_followers_for_a_channel**](ChannelApi.md#fetch_followers_for_a_channel) | **GET** /farcaster/channel/followers | For channel
[**fetch_relevant_followers_for_a_channel**](ChannelApi.md#fetch_relevant_followers_for_a_channel) | **GET** /farcaster/channel/followers/relevant | Relevant followers
[**fetch_trending_channels**](ChannelApi.md#fetch_trending_channels) | **GET** /farcaster/channel/trending | Channels by activity
[**fetch_user_channel_memberships**](ChannelApi.md#fetch_user_channel_memberships) | **GET** /farcaster/user/memberships/list | Member of
[**fetch_user_channels**](ChannelApi.md#fetch_user_channels) | **GET** /farcaster/user/channels | Following
[**fetch_users_active_channels**](ChannelApi.md#fetch_users_active_channels) | **GET** /farcaster/channel/user | Fetch channels that user is active in
[**follow_channel**](ChannelApi.md#follow_channel) | **POST** /farcaster/channel/follow | Follow a channel
[**invite_channel_member**](ChannelApi.md#invite_channel_member) | **POST** /farcaster/channel/member/invite | Invite
[**lookup_channel**](ChannelApi.md#lookup_channel) | **GET** /farcaster/channel | By ID or parent_url
[**remove_channel_member**](ChannelApi.md#remove_channel_member) | **DELETE** /farcaster/channel/member | Remove user
[**respond_channel_invite**](ChannelApi.md#respond_channel_invite) | **PUT** /farcaster/channel/member/invite | Accept or reject an invite
[**search_channels**](ChannelApi.md#search_channels) | **GET** /farcaster/channel/search | Search by ID or name
[**unfollow_channel**](ChannelApi.md#unfollow_channel) | **DELETE** /farcaster/channel/follow | Unfollow a channel



## fetch_all_channels

> models::ChannelListResponse fetch_all_channels(limit, cursor)
Fetch all channels with their details

Returns a list of all channels with their details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ChannelListResponse**](ChannelListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_bulk_channels

> models::ChannelResponseBulk fetch_bulk_channels(ids, r#type, viewer_fid)
Bulk fetch

Returns details of multiple channels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma separated list of channel IDs or parent_urls, up to 100 at a time | [required] |
**r#type** | Option<[**ChannelType**](.md)> | Type of identifier being used to query the channels. Defaults to ID. |  |
**viewer_fid** | Option<**i32**> | FID of the user viewing the channels. |  |

### Return type

[**models::ChannelResponseBulk**](ChannelResponseBulk.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_channel_invites

> models::ChannelMemberInviteListResponse fetch_channel_invites(channel_id, invited_fid, limit, cursor)
Open invites

Fetch a list of invites, either in a channel or for a user. If both are provided, open channel invite for that user is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | Option<**String**> | Channel ID for the channel being queried |  |
**invited_fid** | Option<**i32**> | FID of the user being invited |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ChannelMemberInviteListResponse**](ChannelMemberInviteListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_channel_members

> models::ChannelMemberListResponse fetch_channel_members(channel_id, fid, limit, cursor, x_neynar_experimental)
Fetch members

Fetch a list of members in a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel ID for the channel being queried | [required] |
**fid** | Option<**i32**> | FID of the user being queried. Specify this to check if a user is a member of the channel without paginating through all members. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::ChannelMemberListResponse**](ChannelMemberListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_followers_for_a_channel

> models::UsersResponse fetch_followers_for_a_channel(id, viewer_fid, cursor, limit, x_neynar_experimental)
For channel

Returns a list of followers for a specific channel. Max limit is 1000. Use cursor for pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Channel ID for the channel being queried | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return a list of followers that respects this user's mutes and blocks and includes `viewer_context`. |  |
**cursor** | Option<**String**> | Pagination cursor. |  |
**limit** | Option<**i32**> | Number of followers to fetch |  |[default to 25]
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UsersResponse**](UsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_relevant_followers_for_a_channel

> models::RelevantFollowersResponse fetch_relevant_followers_for_a_channel(id, viewer_fid, x_neynar_experimental)
Relevant followers

Returns a list of relevant channel followers for a specific FID. This usually shows on a channel as \"X, Y, Z follow this channel\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Channel ID being queried | [required] |
**viewer_fid** | **i32** | The FID of the user to customize this response for. Providing this will also return a list of followers that respects this user's mutes and blocks and includes `viewer_context`. | [required] |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::RelevantFollowersResponse**](RelevantFollowersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_trending_channels

> models::TrendingChannelResponse fetch_trending_channels(time_window, limit, cursor)
Channels by activity

Returns a list of trending channels based on activity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_window** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 10]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::TrendingChannelResponse**](TrendingChannelResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_channel_memberships

> models::ChannelMemberListResponse fetch_user_channel_memberships(fid, limit, cursor)
Member of

Returns a list of all channels with their details that an FID is a member of. Data may have a delay of up to 1 hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the user. | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ChannelMemberListResponse**](ChannelMemberListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_channels

> models::ChannelListResponse fetch_user_channels(fid, limit, cursor)
Following

Returns a list of all channels with their details that a FID follows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID of the user. | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ChannelListResponse**](ChannelListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_active_channels

> models::UsersActiveChannelsResponse fetch_users_active_channels(fid, limit, cursor)
Fetch channels that user is active in

Fetches all channels that a user has casted in, in reverse chronological order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The user's FID (identifier) | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::UsersActiveChannelsResponse**](UsersActiveChannelsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_channel

> models::OperationResponse follow_channel(channel_follow_req_body)
Follow a channel

Follow a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_follow_req_body** | [**ChannelFollowReqBody**](ChannelFollowReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_channel_member

> models::OperationResponse invite_channel_member(invite_channel_member_req_body)
Invite

Invite a user to a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_channel_member_req_body** | [**InviteChannelMemberReqBody**](InviteChannelMemberReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_channel

> models::ChannelResponse lookup_channel(id, r#type, viewer_fid)
By ID or parent_url

Returns details of a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Channel ID for the channel being queried | [required] |
**r#type** | Option<[**ChannelType**](.md)> | Type of identifier being used to query the channel. Defaults to ID. |  |
**viewer_fid** | Option<**i32**> | FID of the user viewing the channel. |  |

### Return type

[**models::ChannelResponse**](ChannelResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_channel_member

> models::OperationResponse remove_channel_member(remove_channel_member_req_body)
Remove user

Remove a user from a channel or a user's invite to a channel role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_channel_member_req_body** | [**RemoveChannelMemberReqBody**](RemoveChannelMemberReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## respond_channel_invite

> models::OperationResponse respond_channel_invite(respond_channel_invite_req_body)
Accept or reject an invite

Accept or reject a channel invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**respond_channel_invite_req_body** | [**RespondChannelInviteReqBody**](RespondChannelInviteReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_channels

> models::ChannelSearchResponse search_channels(q, limit, cursor)
Search by ID or name

Returns a list of channels based on ID or name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Channel ID or name for the channel being queried | [required] |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 20]
**cursor** | Option<**String**> | Pagination cursor. |  |

### Return type

[**models::ChannelSearchResponse**](ChannelSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfollow_channel

> models::OperationResponse unfollow_channel(channel_follow_req_body)
Unfollow a channel

Unfollow a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_follow_req_body** | [**ChannelFollowReqBody**](ChannelFollowReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

