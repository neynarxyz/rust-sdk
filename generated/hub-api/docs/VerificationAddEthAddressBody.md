# VerificationAddEthAddressBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | The Ethereum (0x-prefixed) or Solana address that the user is claiming ownership of. Must match the address that produced the signature. | 
**eth_signature** | **String** | Base64-encoded signature produced by the blockchain address, proving ownership. For Ethereum, this is an ECDSA signature of a specific message format. | 
**block_hash** | **String** | The hash of the most recent block when the signature was created. Used to verify the approximate time of signature creation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


