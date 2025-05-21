# RegisterUserOnChainReqBodyPreRegistrationCallsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | **String** | Ethereum address | 
**value** | Option<**i32**> | Value in wei to send with the transaction. This is not the amount of ETH that will be sent, but rather the value of the transaction. | [optional][default to 0]
**data** | **String** | Hexadecimal number expressed as string with '0x' prefix | 
**allow_failure** | Option<**bool**> | Set it to true if you want to ignore the failure of this call. If set to false, the registration will fail if this call fails. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


