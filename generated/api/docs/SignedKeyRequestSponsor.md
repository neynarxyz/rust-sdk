# SignedKeyRequestSponsor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fid** | Option<**i32**> | The unique identifier of a farcaster user or app (unsigned integer) | [optional]
**signature** | Option<**String**> | Signature generated by the fid of the sponsor and the signature generated from signKeyRequest for the app. | [optional]
**sponsored_by_neynar** | Option<**bool**> | Neynar will sponsor the signer if set to true. **Note: ** If sponsor.fid and sponsor.signature are provided along with sponsored_by_neynar set to true,  the sponsor.fid and sponsor.signature will be ignored.  Neynar will sponsor the signer on behalf of the user. The developer will get charged in compute units.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


