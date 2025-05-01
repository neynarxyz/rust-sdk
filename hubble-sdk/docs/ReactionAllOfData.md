# ReactionAllOfData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fid** | **i32** | The unique identifier (FID) of the user who created this message. FIDs are assigned sequentially when users register on the network and cannot be changed. | 
**timestamp** | **i64** | Seconds since Farcaster Epoch (2021-01-01T00:00:00Z). Used to order messages chronologically and determine the most recent state. Must be within 10 minutes of the current time when the message is created. | 
**network** | [**models::FarcasterNetwork**](FarcasterNetwork.md) |  | 
**reaction_body** | [**models::ReactionBody**](ReactionBody.md) | Contains the type of reaction (like/recast) and the target content being reacted to. The target can be specified either by castId or URL. | 
**r#type** | [**models::MessageType**](MessageType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


