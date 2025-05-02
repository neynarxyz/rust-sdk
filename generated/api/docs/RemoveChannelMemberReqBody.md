# RemoveChannelMemberReqBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signer_uuid** | **String** | UUID of the signer. `signer_uuid` is paired with API key, can't use a `uuid` made with a different API key.  | 
**channel_id** | **String** | The unique identifier of a farcaster channel | 
**fid** | **i32** | The unique identifier of a farcaster user (unsigned integer) | 
**role** | [**models::ChannelMemberRole**](ChannelMemberRole.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


