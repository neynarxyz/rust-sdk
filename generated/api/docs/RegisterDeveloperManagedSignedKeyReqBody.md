# RegisterDeveloperManagedSignedKeyReqBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_key** | **String** | Ed25519 public key | 
**signature** | **String** | Signature generated by the custody address of the app. Signed data includes app_fid, deadline, signer’s public key | 
**app_fid** | **i32** | The unique identifier of a farcaster user or app (unsigned integer) | 
**deadline** | **i32** | unix timestamp in seconds that controls how long the signed key request is valid for. (24 hours from now is recommended) | 
**redirect_url** | Option<**String**> | Url to redirect to after the signer is approved.  **Note** : This should only be used when requesting a signer from a native mobile application.  | [optional]
**sponsor** | Option<[**models::SignedKeyRequestSponsor**](SignedKeyRequestSponsor.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


