# \UsernamesApi

All URIs are relative to *https://hub-api.neynar.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_username_proof_by_name**](UsernamesApi.md#fetch_username_proof_by_name) | **GET** /v1/userNameProofByName | Proof for a username
[**fetch_username_proofs_by_fid**](UsernamesApi.md#fetch_username_proofs_by_fid) | **GET** /v1/userNameProofsByFid | Proofs provided by an FID



## fetch_username_proof_by_name

> models::UserNameProof fetch_username_proof_by_name(name)
Proof for a username

Fetch a proof for a username.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The Farcaster username or ENS address | [required] |

### Return type

[**models::UserNameProof**](UserNameProof.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_username_proofs_by_fid

> models::UsernameProofsResponse fetch_username_proofs_by_fid(fid)
Proofs provided by an FID

Fetch proofs provided by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fid** | **i32** | The FID being requested | [required] |

### Return type

[**models::UsernameProofsResponse**](UsernameProofsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

