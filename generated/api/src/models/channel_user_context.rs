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

/// ChannelUserContext : Adds context on the viewer's or author's role in the channel.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelUserContext {
    /// Indicates if the user is following the channel.
    #[serde(rename = "following")]
    pub following: bool,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<models::ChannelMemberRole>,
}

impl ChannelUserContext {
    /// Adds context on the viewer's or author's role in the channel.
    pub fn new(following: bool) -> ChannelUserContext {
        ChannelUserContext {
            following,
            role: None,
        }
    }
}
