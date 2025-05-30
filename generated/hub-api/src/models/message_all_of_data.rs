/*
 * Farcaster Hub API
 *
 * Perform basic queries of Farcaster state via the REST API of a Farcaster hub. See the [Neynar docs](https://docs.neynar.com/reference) for more details.
 *
 * The version of the OpenAPI document: 2.35.0
 * Contact: team@neynar.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageAllOfData {
    #[serde(rename = "MESSAGE_TYPE_CAST_ADD")]
    MessageTypeCastAdd(Box<models::MessageDataCastAdd>),
    #[serde(rename = "MESSAGE_TYPE_CAST_REMOVE")]
    MessageTypeCastRemove(Box<models::MessageDataCastRemove>),
    #[serde(rename = "MESSAGE_TYPE_REACTION_ADD")]
    MessageTypeReactionAdd(Box<models::MessageDataReaction>),
    #[serde(rename = "MESSAGE_TYPE_LINK_ADD")]
    MessageTypeLinkAdd(Box<models::MessageDataLink>),
    #[serde(rename = "MESSAGE_TYPE_VERIFICATION_ADD_ETH_ADDRESS")]
    MessageTypeVerificationAddEthAddress(Box<models::MessageDataVerificationAdd>),
    #[serde(rename = "MESSAGE_TYPE_VERIFICATION_REMOVE")]
    MessageTypeVerificationRemove(Box<models::MessageDataVerificationRemove>),
    #[serde(rename = "MESSAGE_TYPE_USER_DATA_ADD")]
    MessageTypeUserDataAdd(Box<models::MessageDataUserDataAdd>),
    #[serde(rename = "MESSAGE_TYPE_USERNAME_PROOF")]
    MessageTypeUsernameProof(Box<models::MessageDataUsernameProof>),
    #[serde(rename = "MESSAGE_TYPE_FRAME_ACTION")]
    MessageTypeFrameAction(Box<models::MessageDataFrameAction>),
}

impl Default for MessageAllOfData {
    fn default() -> Self {
        Self::MessageTypeCastAdd(Default::default())
    }
}
