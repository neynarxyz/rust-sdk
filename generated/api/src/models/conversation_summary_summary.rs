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
pub struct ConversationSummarySummary {
    /// Summary generated by an LLM
    #[serde(rename = "text")]
    pub text: String,
    /// Users who casted in a conversation thread
    #[serde(rename = "participants")]
    pub participants: Vec<models::User>,
    /// Users who were mentioned in a conversation thread
    #[serde(rename = "mentioned_profiles")]
    pub mentioned_profiles: Vec<models::User>,
}

impl ConversationSummarySummary {
    pub fn new(
        text: String,
        participants: Vec<models::User>,
        mentioned_profiles: Vec<models::User>,
    ) -> ConversationSummarySummary {
        ConversationSummarySummary {
            text,
            participants,
            mentioned_profiles,
        }
    }
}
