# Rust API client for neynar_hub_sdk

Perform basic queries of Farcaster state via the REST API of a Farcaster hub. See the [Neynar docs](https://docs.neynar.com/reference) for more details.


For more information, please visit [https://neynar.com/](https://neynar.com/)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.35.0
- Package version: 2.46.0
- Generator version: 7.13.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `neynar_hub_sdk` and add the following to `Cargo.toml` under `[dependencies]`:

```
neynar_hub_sdk = { path = "./neynar_hub_sdk" }
```

## Documentation for API Endpoints

All URIs are relative to *https://hub-api.neynar.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CastsApi* | [**fetch_casts_by_parent**](docs/CastsApi.md#fetch_casts_by_parent) | **GET** /v1/castsByParent | By parent cast
*CastsApi* | [**fetch_casts_mentioning_user**](docs/CastsApi.md#fetch_casts_mentioning_user) | **GET** /v1/castsByMention | Mentioning an FID
*CastsApi* | [**fetch_users_casts**](docs/CastsApi.md#fetch_users_casts) | **GET** /v1/castsByFid | By FID
*CastsApi* | [**lookup_cast_by_hash_and_fid**](docs/CastsApi.md#lookup_cast_by_hash_and_fid) | **GET** /v1/castById | By FID and Hash
*FidsApi* | [**fetch_fids**](docs/FidsApi.md#fetch_fids) | **GET** /v1/fids | Fetch a list of all the FIDs
*HubEventsApi* | [**fetch_events**](docs/HubEventsApi.md#fetch_events) | **GET** /v1/events | Page of events
*HubEventsApi* | [**lookup_event**](docs/HubEventsApi.md#lookup_event) | **GET** /v1/eventById | Event by ID
*InfoApi* | [**lookup_hub_info**](docs/InfoApi.md#lookup_hub_info) | **GET** /v1/info | Sync Methods
*LinksApi* | [**fetch_user_followers**](docs/LinksApi.md#fetch_user_followers) | **GET** /v1/linksByTargetFid | To target FID
*LinksApi* | [**fetch_user_following**](docs/LinksApi.md#fetch_user_following) | **GET** /v1/linksByFid | From source FID
*LinksApi* | [**lookup_user_relation**](docs/LinksApi.md#lookup_user_relation) | **GET** /v1/linkById | By its FID and target FID
*MessageApi* | [**publish_message**](docs/MessageApi.md#publish_message) | **POST** /v1/submitMessage | Submit signed message
*MessageApi* | [**validate_message**](docs/MessageApi.md#validate_message) | **POST** /v1/validateMessage | Validate signed message
*OnChainEventsApi* | [**fetch_user_on_chain_events**](docs/OnChainEventsApi.md#fetch_user_on_chain_events) | **GET** /v1/onChainEventsByFid | Fetch a list of on-chain events provided by an FID
*OnChainEventsApi* | [**fetch_user_on_chain_signers_events**](docs/OnChainEventsApi.md#fetch_user_on_chain_signers_events) | **GET** /v1/onChainSignersByFid | Fetch a list of signers provided by an FID
*OnChainEventsApi* | [**lookup_on_chain_id_registry_event_by_address**](docs/OnChainEventsApi.md#lookup_on_chain_id_registry_event_by_address) | **GET** /v1/onChainIdRegistryEventByAddress | Fetch an on-chain ID Registry Event for a given Address
*ReactionsApi* | [**fetch_cast_reactions**](docs/ReactionsApi.md#fetch_cast_reactions) | **GET** /v1/reactionsByCast | On cast
*ReactionsApi* | [**fetch_reactions_by_target**](docs/ReactionsApi.md#fetch_reactions_by_target) | **GET** /v1/reactionsByTarget | To a target URL
*ReactionsApi* | [**fetch_user_reactions**](docs/ReactionsApi.md#fetch_user_reactions) | **GET** /v1/reactionsByFid | By FID
*ReactionsApi* | [**lookup_reaction_by_id**](docs/ReactionsApi.md#lookup_reaction_by_id) | **GET** /v1/reactionById | By FID or cast
*StorageApi* | [**lookup_user_storage_limit**](docs/StorageApi.md#lookup_user_storage_limit) | **GET** /v1/storageLimitsByFid | FID's limits
*UserDataApi* | [**fetch_user_data**](docs/UserDataApi.md#fetch_user_data) | **GET** /v1/userDataByFid | Fetch UserData for a FID
*UsernamesApi* | [**fetch_username_proof_by_name**](docs/UsernamesApi.md#fetch_username_proof_by_name) | **GET** /v1/userNameProofByName | Proof for a username
*UsernamesApi* | [**fetch_username_proofs_by_fid**](docs/UsernamesApi.md#fetch_username_proofs_by_fid) | **GET** /v1/userNameProofsByFid | Proofs provided by an FID
*VerificationsApi* | [**fetch_verifications_by_fid**](docs/VerificationsApi.md#fetch_verifications_by_fid) | **GET** /v1/verificationsByFid | Provided by an FID


## Documentation For Models

 - [CastAdd](docs/CastAdd.md)
 - [CastAddAllOfData](docs/CastAddAllOfData.md)
 - [CastAddBody](docs/CastAddBody.md)
 - [CastEmbed](docs/CastEmbed.md)
 - [CastId](docs/CastId.md)
 - [DbStats](docs/DbStats.md)
 - [Embed](docs/Embed.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ErrorResponseMetadata](docs/ErrorResponseMetadata.md)
 - [FarcasterNetwork](docs/FarcasterNetwork.md)
 - [FetchCastReactions200Response](docs/FetchCastReactions200Response.md)
 - [FetchCastsByParent200Response](docs/FetchCastsByParent200Response.md)
 - [FetchCastsMentioningUser200Response](docs/FetchCastsMentioningUser200Response.md)
 - [FetchEvents200Response](docs/FetchEvents200Response.md)
 - [FetchReactionsByTarget200Response](docs/FetchReactionsByTarget200Response.md)
 - [FetchUserData200Response](docs/FetchUserData200Response.md)
 - [FetchUserData200ResponseOneOf](docs/FetchUserData200ResponseOneOf.md)
 - [FetchUserFollowers200Response](docs/FetchUserFollowers200Response.md)
 - [FetchUserFollowing200Response](docs/FetchUserFollowing200Response.md)
 - [FetchUserOnChainEvents200Response](docs/FetchUserOnChainEvents200Response.md)
 - [FetchUserOnChainSignersEvents200Response](docs/FetchUserOnChainSignersEvents200Response.md)
 - [FetchUserOnChainSignersEvents200ResponseOneOf](docs/FetchUserOnChainSignersEvents200ResponseOneOf.md)
 - [FetchUserReactions200Response](docs/FetchUserReactions200Response.md)
 - [FetchUsersCasts200Response](docs/FetchUsersCasts200Response.md)
 - [FetchVerificationsByFid200Response](docs/FetchVerificationsByFid200Response.md)
 - [FidsResponse](docs/FidsResponse.md)
 - [FrameActionBody](docs/FrameActionBody.md)
 - [HashScheme](docs/HashScheme.md)
 - [HubEvent](docs/HubEvent.md)
 - [HubEventMergeMessage](docs/HubEventMergeMessage.md)
 - [HubEventMergeOnChainEvent](docs/HubEventMergeOnChainEvent.md)
 - [HubEventMergeUsernameProof](docs/HubEventMergeUsernameProof.md)
 - [HubEventPruneMessage](docs/HubEventPruneMessage.md)
 - [HubEventRevokeMessage](docs/HubEventRevokeMessage.md)
 - [HubInfoResponse](docs/HubInfoResponse.md)
 - [IdRegisterEventBody](docs/IdRegisterEventBody.md)
 - [IdRegisterEventType](docs/IdRegisterEventType.md)
 - [LinkAdd](docs/LinkAdd.md)
 - [LinkAddAllOfData](docs/LinkAddAllOfData.md)
 - [LinkBody](docs/LinkBody.md)
 - [LinkType](docs/LinkType.md)
 - [MergeMessageBody](docs/MergeMessageBody.md)
 - [MergeOnChainEventBody](docs/MergeOnChainEventBody.md)
 - [MergeUserNameProofBody](docs/MergeUserNameProofBody.md)
 - [Message](docs/Message.md)
 - [MessageAllOfData](docs/MessageAllOfData.md)
 - [MessageCommon](docs/MessageCommon.md)
 - [MessageDataCastAdd](docs/MessageDataCastAdd.md)
 - [MessageDataCastRemove](docs/MessageDataCastRemove.md)
 - [MessageDataCommon](docs/MessageDataCommon.md)
 - [MessageDataFrameAction](docs/MessageDataFrameAction.md)
 - [MessageDataLink](docs/MessageDataLink.md)
 - [MessageDataReaction](docs/MessageDataReaction.md)
 - [MessageDataUserDataAdd](docs/MessageDataUserDataAdd.md)
 - [MessageDataUsernameProof](docs/MessageDataUsernameProof.md)
 - [MessageDataVerificationAdd](docs/MessageDataVerificationAdd.md)
 - [MessageDataVerificationRemove](docs/MessageDataVerificationRemove.md)
 - [MessageType](docs/MessageType.md)
 - [OnChainEvent](docs/OnChainEvent.md)
 - [OnChainEventCommon](docs/OnChainEventCommon.md)
 - [OnChainEventIdRegister](docs/OnChainEventIdRegister.md)
 - [OnChainEventSigner](docs/OnChainEventSigner.md)
 - [OnChainEventSignerMigrated](docs/OnChainEventSignerMigrated.md)
 - [OnChainEventStorageRent](docs/OnChainEventStorageRent.md)
 - [OnChainEventType](docs/OnChainEventType.md)
 - [PaginationResponse](docs/PaginationResponse.md)
 - [PruneMessageBody](docs/PruneMessageBody.md)
 - [Reaction](docs/Reaction.md)
 - [ReactionAllOfData](docs/ReactionAllOfData.md)
 - [ReactionBody](docs/ReactionBody.md)
 - [ReactionType](docs/ReactionType.md)
 - [RevokeMessageBody](docs/RevokeMessageBody.md)
 - [SignatureScheme](docs/SignatureScheme.md)
 - [SignerEventBody](docs/SignerEventBody.md)
 - [SignerEventType](docs/SignerEventType.md)
 - [SignerMigratedEventBody](docs/SignerMigratedEventBody.md)
 - [StorageLimit](docs/StorageLimit.md)
 - [StorageLimitsResponse](docs/StorageLimitsResponse.md)
 - [StorageRentEventBody](docs/StorageRentEventBody.md)
 - [StoreType](docs/StoreType.md)
 - [UrlEmbed](docs/UrlEmbed.md)
 - [UserDataAdd](docs/UserDataAdd.md)
 - [UserDataAddAllOfData](docs/UserDataAddAllOfData.md)
 - [UserDataBody](docs/UserDataBody.md)
 - [UserDataType](docs/UserDataType.md)
 - [UserNameProof](docs/UserNameProof.md)
 - [UserNameType](docs/UserNameType.md)
 - [UsernameProofsResponse](docs/UsernameProofsResponse.md)
 - [ValidateMessageResponse](docs/ValidateMessageResponse.md)
 - [Verification](docs/Verification.md)
 - [VerificationAddEthAddressBody](docs/VerificationAddEthAddressBody.md)
 - [VerificationAllOfData](docs/VerificationAllOfData.md)
 - [VerificationRemoveBody](docs/VerificationRemoveBody.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

team@neynar.com

