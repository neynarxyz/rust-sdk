# \UserApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_verification**](UserApi.md#delete_verification) | **DELETE** /farcaster/user/verification | Delete verification
[**fetch_bulk_users**](UserApi.md#fetch_bulk_users) | **GET** /farcaster/user/bulk | By FIDs
[**fetch_bulk_users_by_eth_or_sol_address**](UserApi.md#fetch_bulk_users_by_eth_or_sol_address) | **GET** /farcaster/user/bulk-by-address | By Eth or Sol addresses
[**fetch_power_users**](UserApi.md#fetch_power_users) | **GET** /farcaster/user/power | Power users
[**fetch_power_users_lite**](UserApi.md#fetch_power_users_lite) | **GET** /farcaster/user/power_lite | Power user FIDs
[**fetch_users_by_location**](UserApi.md#fetch_users_by_location) | **GET** /farcaster/user/by_location | By location
[**follow_user**](UserApi.md#follow_user) | **POST** /farcaster/user/follow | Follow user
[**get_fresh_account_fid**](UserApi.md#get_fresh_account_fid) | **GET** /farcaster/user/fid | Fetch fresh FID
[**lookup_user_by_custody_address**](UserApi.md#lookup_user_by_custody_address) | **GET** /farcaster/user/custody-address | By custody-address
[**lookup_user_by_username**](UserApi.md#lookup_user_by_username) | **GET** /farcaster/user/by_username | By username
[**lookup_users_by_x_username**](UserApi.md#lookup_users_by_x_username) | **GET** /farcaster/user/by_x_username | By X username
[**publish_verification**](UserApi.md#publish_verification) | **POST** /farcaster/user/verification | Add verification
[**register_account**](UserApi.md#register_account) | **POST** /farcaster/user | Register new account
[**search_user**](UserApi.md#search_user) | **GET** /farcaster/user/search | Search for Usernames
[**unfollow_user**](UserApi.md#unfollow_user) | **DELETE** /farcaster/user/follow | Unfollow user
[**update_user**](UserApi.md#update_user) | **PATCH** /farcaster/user | Update user profile



## delete_verification

> models::OperationResponse delete_verification(remove_verification_req_body)
Delete verification

Removes verification for an eth address for the user \\ (In order to delete verification `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_verification_req_body** | [**RemoveVerificationReqBody**](RemoveVerificationReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_bulk_users

> models::BulkUsersResponse fetch_bulk_users(fids, viewer_fid, x_neynar_experimental)
By FIDs

Fetches information about multiple users based on FIDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fids** | **String** | Comma separated list of FIDs, up to 100 at a time | [required] |
**viewer_fid** | Option<**i32**> |  |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::BulkUsersResponse**](BulkUsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_bulk_users_by_eth_or_sol_address

> std::collections::HashMap<String, Vec<models::User>> fetch_bulk_users_by_eth_or_sol_address(addresses, address_types, viewer_fid, x_neynar_experimental)
By Eth or Sol addresses

Fetches all users based on multiple Ethereum or Solana addresses.  Each farcaster user has a custody Ethereum address and optionally verified Ethereum or Solana addresses. This endpoint returns all users that have any of the given addresses as their custody or verified Ethereum or Solana addresses.  A custody address can be associated with only 1 farcaster user at a time but a verified address can be associated with multiple users. You can pass in Ethereum and Solana addresses, comma separated, in the same request. The response will contain users associated with the given addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addresses** | **String** | Comma separated list of Ethereum addresses, up to 350 at a time | [required] |
**address_types** | Option<[**Vec<models::BulkUserAddressType>**](models::BulkUserAddressType.md)> | Customize which address types the request should search for. This is a comma-separated string that can include the following values: 'custody_address' and 'verified_address'. By default api returns both. To select multiple types, use a comma-separated list of these values.  |  |
**viewer_fid** | Option<**i32**> |  |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**std::collections::HashMap<String, Vec<models::User>>**](Vec.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_power_users

> models::UsersResponse fetch_power_users(viewer_fid, limit, cursor, x_neynar_experimental)
Power users

Fetches power users based on Warpcast power badges. Information is updated once a day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**viewer_fid** | Option<**i32**> |  |  |
**limit** | Option<**i32**> | Number of power users to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UsersResponse**](UsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_power_users_lite

> models::UserPowerLiteResponse fetch_power_users_lite(x_neynar_experimental)
Power user FIDs

Fetches power users and respond in a backwards compatible format to Warpcast's deprecated power badge endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UserPowerLiteResponse**](UserPowerLiteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_by_location

> models::UsersResponse fetch_users_by_location(latitude, longitude, viewer_fid, limit, cursor, x_neynar_experimental)
By location

Fetches a list of users given a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**latitude** | **f64** | Latitude of the location | [required] |
**longitude** | **f64** | Longitude of the location | [required] |
**viewer_fid** | Option<**i32**> | FID of the user viewing the feed. Providing this will return a list of users that respects this user's mutes and blocks and includes `viewer_context`. |  |
**limit** | Option<**i32**> | Number of results to fetch |  |[default to 25]
**cursor** | Option<**String**> | Pagination cursor |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UsersResponse**](UsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_user

> models::BulkFollowResponse follow_user(follow_req_body)
Follow user

Follow a user \\ (In order to follow a user `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**follow_req_body** | [**FollowReqBody**](FollowReqBody.md) |  | [required] |

### Return type

[**models::BulkFollowResponse**](BulkFollowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fresh_account_fid

> models::UserFidResponse get_fresh_account_fid(x_neynar_experimental)
Fetch fresh FID

Fetches FID to [assign it to new user](https://docs.neynar.com/reference/register-account)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UserFidResponse**](UserFIDResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_user_by_custody_address

> models::UserResponse lookup_user_by_custody_address(custody_address)
By custody-address

Lookup a user by custody-address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custody_address** | **String** | Custody Address associated with mnemonic | [required] |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_user_by_username

> models::UserResponse lookup_user_by_username(username, viewer_fid, x_neynar_experimental)
By username

Fetches a single hydrated user object given a username

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username of the user to fetch | [required] |
**viewer_fid** | Option<**i32**> |  |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_users_by_x_username

> models::BulkUsersResponse lookup_users_by_x_username(x_username, viewer_fid, x_neynar_experimental)
By X username

Fetches the users who have verified the specified X (Twitter) username

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_username** | **String** | X (Twitter) username to search for, without the @ symbol | [required] |
**viewer_fid** | Option<**i32**> | FID of the viewer for contextual information like follows and blocks |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::BulkUsersResponse**](BulkUsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_verification

> models::OperationResponse publish_verification(add_verification_req_body)
Add verification

Adds verification for an eth address or contract for the user \\ (In order to add verification `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_verification_req_body** | [**AddVerificationReqBody**](AddVerificationReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_account

> models::RegisterUserResponse register_account(register_user_req_body)
Register new account

Register account on farcaster.  **Note:** This API must be called within 10 minutes of the fetch FID API call (i.e., /v2/farcaster/user/fid). Otherwise, Neynar will assign this FID to another available user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_user_req_body** | [**RegisterUserReqBody**](RegisterUserReqBody.md) |  | [required] |

### Return type

[**models::RegisterUserResponse**](RegisterUserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_user

> models::UserSearchResponse search_user(q, viewer_fid, limit, cursor, x_neynar_experimental)
Search for Usernames

Search for Usernames

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**viewer_fid** | Option<**i32**> | Providing this will return search results that respects this user's mutes and blocks and includes `viewer_context`. |  |
**limit** | Option<**i32**> | Number of users to fetch |  |[default to 5]
**cursor** | Option<**String**> | Pagination cursor. |  |
**x_neynar_experimental** | Option<**bool**> | Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details. |  |[default to false]

### Return type

[**models::UserSearchResponse**](UserSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfollow_user

> models::BulkFollowResponse unfollow_user(follow_req_body)
Unfollow user

Unfollow a user \\ (In order to unfollow a user `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**follow_req_body** | [**FollowReqBody**](FollowReqBody.md) |  | [required] |

### Return type

[**models::BulkFollowResponse**](BulkFollowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::OperationResponse update_user(update_user_req_body)
Update user profile

Update user profile \\ (In order to update user's profile `signer_uuid` must be approved) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_req_body** | [**UpdateUserReqBody**](UpdateUserReqBody.md) |  | [required] |

### Return type

[**models::OperationResponse**](OperationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

