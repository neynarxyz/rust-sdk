# \OnchainApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_fungible**](OnchainApi.md#deploy_fungible) | **POST** /fungible | Deploy fungible
[**fetch_relevant_fungible_owners**](OnchainApi.md#fetch_relevant_fungible_owners) | **GET** /farcaster/fungible/owner/relevant | Relevant owners
[**fetch_user_balance**](OnchainApi.md#fetch_user_balance) | **GET** /farcaster/user/balance | Token balance
[**register_account_onchain**](OnchainApi.md#register_account_onchain) | **POST** /farcaster/user/register | Register account onchain



## deploy_fungible

> models::DeployFungibleResponse deploy_fungible(owner, symbol, name, metadata_left_square_bracket_media_right_square_bracket, metadata_left_square_bracket_description_right_square_bracket, metadata_left_square_bracket_nsfw_right_square_bracket, metadata_left_square_bracket_website_link_right_square_bracket, metadata_left_square_bracket_twitter_right_square_bracket, metadata_left_square_bracket_discord_right_square_bracket, metadata_left_square_bracket_telegram_right_square_bracket, network, factory)
Deploy fungible

Creates a new token. This is an allowlisted API, reach out if you want access. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | Ethereum address of the one who is creating the token | [required] |
**symbol** | **String** | Symbol/Ticker for the token | [required] |
**name** | **String** | Name of the token | [required] |
**metadata_left_square_bracket_media_right_square_bracket** | Option<**Vec<u8>**> | Media file associated with the token.  Supported formats are image/jpeg, image/gif and image/png  |  |
**metadata_left_square_bracket_description_right_square_bracket** | Option<**String**> | Description of the token |  |
**metadata_left_square_bracket_nsfw_right_square_bracket** | Option<**String**> | Indicates if the token is NSFW (Not Safe For Work).  |  |
**metadata_left_square_bracket_website_link_right_square_bracket** | Option<**String**> | Website link related to the token |  |
**metadata_left_square_bracket_twitter_right_square_bracket** | Option<**String**> | Twitter profile link |  |
**metadata_left_square_bracket_discord_right_square_bracket** | Option<**String**> | Discord server link |  |
**metadata_left_square_bracket_telegram_right_square_bracket** | Option<**String**> | Telegram link |  |
**network** | Option<**String**> | Network/Chain name |  |[default to base]
**factory** | Option<**String**> | Factory name - wow -> [wow.xyz](https://wow.xyz) - clanker -> [clanker.world](https://www.clanker.world)  |  |[default to wow]

### Return type

[**models::DeployFungibleResponse**](DeployFungibleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_relevant_fungible_owners

> models::RelevantFungibleOwnersResponse fetch_relevant_fungible_owners(contract_address, networks, viewer_fid)
Relevant owners

Fetch a list of relevant owners for a specific FID. This usually shows on a fungible asset page as \"X, Y, Z and N others you know own this asset\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** | Contract address of the fungible asset | [required] |
**networks** | [**Vec<models::Networks>**](models::Networks.md) | Comma separated list of networks to fetch balances for. Currently, only \"base\" is supported. | [required] |
**viewer_fid** | Option<**i32**> | If you provide a viewer_fid, the response will include token holders from the user's network, respecting their mutes and blocks and including viewer_context; if not provided, the response will show top token holders across the network—both sets can be combined to generate a longer list if desired. |  |

### Return type

[**models::RelevantFungibleOwnersResponse**](RelevantFungibleOwnersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_balance

> models::BalanceResponse fetch_user_balance(fid, networks)
Token balance

Fetches the token balances of a user given their FID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | FID of the user to fetch | [required] |
**networks** | [**Vec<models::Networks>**](models::Networks.md) | Comma separated list of networks to fetch balances for. Currently, only \"base\" is supported. | [required] |

### Return type

[**models::BalanceResponse**](BalanceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_account_onchain

> models::RegisterUserOnChainResponse register_account_onchain(register_user_on_chain_req_body)
Register account onchain

Register a new farcaster account onchain. Optionally you can pass in signers along to register a new account and create multiple signers in a single transaction 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_user_on_chain_req_body** | [**RegisterUserOnChainReqBody**](RegisterUserOnChainReqBody.md) |  | [required] |

### Return type

[**models::RegisterUserOnChainResponse**](RegisterUserOnChainResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

