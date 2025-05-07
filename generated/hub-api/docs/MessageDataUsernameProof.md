# MessageDataUsernameProof

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**models::MessageType**](MessageType.md)> |  | [optional]
**fid** | **i32** | The unique identifier (FID) of the user who created this message. FIDs are assigned sequentially when users register on the network and cannot be changed. | 
**timestamp** | **i64** | Seconds since Farcaster Epoch (2021-01-01T00:00:00Z). Used to order messages chronologically and determine the most recent state. Must be within 10 minutes of the current time when the message is created. | 
**network** | [**models::FarcasterNetwork**](FarcasterNetwork.md) |  | 
**username_proof_body** | [**models::UserNameProof**](UserNameProof.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


