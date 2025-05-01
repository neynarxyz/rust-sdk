# \SignerApi

All URIs are relative to *https://api.neynar.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_signer**](SignerApi.md#create_signer) | **POST** /farcaster/signer | Create signer
[**fetch_authorization_url**](SignerApi.md#fetch_authorization_url) | **GET** /farcaster/login/authorize | Fetch authorization url
[**fetch_signers**](SignerApi.md#fetch_signers) | **GET** /farcaster/signer/list | List signers
[**lookup_developer_managed_signer**](SignerApi.md#lookup_developer_managed_signer) | **GET** /farcaster/signer/developer_managed | Status by public key
[**lookup_signer**](SignerApi.md#lookup_signer) | **GET** /farcaster/signer | Status
[**publish_message_to_farcaster**](SignerApi.md#publish_message_to_farcaster) | **POST** /farcaster/message | Publish message
[**register_signed_key**](SignerApi.md#register_signed_key) | **POST** /farcaster/signer/signed_key | Register Signed Key
[**register_signed_key_for_developer_managed_signer**](SignerApi.md#register_signed_key_for_developer_managed_signer) | **POST** /farcaster/signer/developer_managed/signed_key | Register Signed Key



## create_signer

> models::Signer create_signer()
Create signer

Creates a signer and returns the signer status. \\ **Note**: While tesing please reuse the signer, it costs money to approve a signer. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Signer**](Signer.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_authorization_url

> models::AuthorizationUrlResponse fetch_authorization_url(client_id, response_type)
Fetch authorization url

Fetch authorization url (Fetched authorized url useful for SIWN login operation)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **uuid::Uuid** |  | [required] |
**response_type** | [**AuthorizationUrlResponseType**](.md) |  | [required] |

### Return type

[**models::AuthorizationUrlResponse**](AuthorizationUrlResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_signers

> models::SignerListResponse fetch_signers(message, signature)
List signers

Fetches a list of signers for a custody address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message** | **String** | A Sign-In with Ethereum (SIWE) message that the user's Ethereum wallet signs. This message includes details such as the domain, address, statement, URI, nonce, and other relevant information following the EIP-4361 standard. It should be structured and URL-encoded.  example:  example.com wants you to sign in with your Ethereum account:\\\\n0x23A...F232\\\\n\\\\nSign in to continue.\\\\n\\\\nURI: example.com\\\\nVersion: 1\\\\nChain ID: 1\\\\nNonce: xyz123\\\\nIssued At: 2021-09-01T14:52:07Z  Note: This is just an example message (So, message is invalid, since we don't want any signers related to NEYNAR_API_DOCS to be exposed).   [Checkout fetch-signers API documentation for more details.](https://docs.neynar.com/docs/fetch-signers-1)  | [required] |
**signature** | **String** | The digital signature produced by signing the provided SIWE message with the user's Ethereum private key. This signature is used to verify the authenticity of the message and the identity of the signer.  | [required] |

### Return type

[**models::SignerListResponse**](SignerListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_developer_managed_signer

> models::DeveloperManagedSigner lookup_developer_managed_signer(public_key)
Status by public key

Fetches the status of a developer managed signer by public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_key** | **String** |  | [required] |

### Return type

[**models::DeveloperManagedSigner**](DeveloperManagedSigner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_signer

> models::Signer lookup_signer(signer_uuid)
Status

Gets information status of a signer by passing in a signer_uuid (Use post API to generate a signer)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signer_uuid** | **String** |  | [required] |

### Return type

[**models::Signer**](Signer.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_message_to_farcaster

> serde_json::Value publish_message_to_farcaster(body)
Publish message

Publish a message to farcaster. The message must be signed by a signer managed by the developer. Use the @farcaster/core library to construct and sign the message. Use the Message.toJSON method on the signed message and pass the JSON in the body of this POST request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_signed_key

> models::Signer register_signed_key(register_signer_key_req_body)
Register Signed Key

Registers an app FID, deadline and a signature. Returns the signer status with an approval url.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_signer_key_req_body** | [**RegisterSignerKeyReqBody**](RegisterSignerKeyReqBody.md) |  | [required] |

### Return type

[**models::Signer**](Signer.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_signed_key_for_developer_managed_signer

> models::DeveloperManagedSigner register_signed_key_for_developer_managed_signer(register_developer_managed_signed_key_req_body)
Register Signed Key

Registers an signed key and returns the developer managed signer status with an approval url.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_developer_managed_signed_key_req_body** | [**RegisterDeveloperManagedSignedKeyReqBody**](RegisterDeveloperManagedSignedKeyReqBody.md) |  | [required] |

### Return type

[**models::DeveloperManagedSigner**](DeveloperManagedSigner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

