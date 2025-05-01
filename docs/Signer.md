# Signer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | Option<**String**> |  | [optional]
**signer_uuid** | **String** | UUID of the signer. `signer_uuid` is paired with API key, can't use a `uuid` made with a different API key.  | 
**public_key** | **String** | Ed25519 public key | 
**status** | **String** |  | 
**signer_approval_url** | Option<**String**> |  | [optional]
**fid** | Option<**i32**> | The unique identifier of a farcaster user (unsigned integer) | [optional]
**permissions** | Option<[**Vec<models::SharedSignerPermission>**](SharedSignerPermission.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


