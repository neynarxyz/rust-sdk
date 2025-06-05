# SearchedUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** |  | 
**fid** | **i32** | The unique identifier of a farcaster user or app (unsigned integer) | 
**username** | **String** |  | 
**display_name** | Option<**String**> |  | [optional]
**custody_address** | **String** | Ethereum address | 
**pfp_url** | Option<**String**> | The URL of the user's profile picture | [optional]
**profile** | [**models::UserProfile**](User_profile.md) |  | 
**follower_count** | **i32** | The number of followers the user has. | 
**following_count** | **i32** | The number of users the user is following. | 
**verifications** | **Vec<String>** |  | 
**verified_addresses** | [**models::UserVerifiedAddresses**](User_verified_addresses.md) |  | 
**verified_accounts** | [**Vec<models::UserVerifiedAccountsInner>**](User_verified_accounts_inner.md) | Verified accounts of the user on other platforms, currently only X is supported. | 
**power_badge** | **bool** |  | 
**experimental** | Option<[**models::UserExperimental**](User_experimental.md)> |  | [optional]
**score** | **f64** | Score that represents the probability that the account is not spam. | 
**viewer_context** | Option<[**models::UserViewerContext**](UserViewerContext.md)> |  | [optional]
**pfp** | [**models::ProfileUrlPfp**](ProfileUrl_pfp.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


