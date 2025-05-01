# AddVerificationReqBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signer_uuid** | **String** | UUID of the signer. `signer_uuid` is paired with API key, can't use a `uuid` made with a different API key.  | 
**address** | **String** | Ethereum address | 
**block_hash** | **String** |  | 
**eth_signature** | **String** |  | 
**verification_type** | Option<[**models::VerificationType**](VerificationType.md)> |  | [optional]
**chain_id** | Option<[**models::VerificationChainId**](VerificationChainId.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


