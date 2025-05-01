# ValidateFrameActionReqBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_bytes_in_hex** | **String** | Hexadecimal string of message bytes. | 
**cast_reaction_context** | Option<**bool**> | Adds viewer_context inside the cast object to indicate whether the interactor reacted to the cast housing the mini app. | [optional][default to true]
**follow_context** | Option<**bool**> | Adds viewer_context inside the user (interactor) object to indicate whether the interactor follows or is followed by the cast author. | [optional][default to false]
**signer_context** | Option<**bool**> | Adds context about the app used by the user inside `frame.action`. | [optional][default to false]
**channel_follow_context** | Option<**bool**> | Adds context about the channel that the cast belongs to inside of the cast object. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


