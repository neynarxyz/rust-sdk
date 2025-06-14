/*
 * Farcaster API V2
 *
 * The Farcaster API allows you to interact with the Farcaster protocol. See the [Neynar docs](https://docs.neynar.com/reference) for more details.
 *
 * The version of the OpenAPI document: 2.46.0
 * Contact: team@neynar.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationConversation {
    #[serde(rename = "cast")]
    pub cast: Box<models::CastWithInteractionsAndConversations>,
    #[serde(
        rename = "chronological_parent_casts",
        skip_serializing_if = "Option::is_none"
    )]
    pub chronological_parent_casts: Option<Vec<models::CastWithInteractions>>,
}

impl ConversationConversation {
    pub fn new(cast: models::CastWithInteractionsAndConversations) -> ConversationConversation {
        ConversationConversation {
            cast: Box::new(cast),
            chronological_parent_casts: None,
        }
    }
}
