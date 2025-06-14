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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "follows")]
    Follows,
    #[serde(rename = "recasts")]
    Recasts,
    #[serde(rename = "likes")]
    Likes,
    #[serde(rename = "mentions")]
    Mentions,
    #[serde(rename = "replies")]
    Replies,
    #[serde(rename = "quotes")]
    Quotes,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Follows => write!(f, "follows"),
            Self::Recasts => write!(f, "recasts"),
            Self::Likes => write!(f, "likes"),
            Self::Mentions => write!(f, "mentions"),
            Self::Replies => write!(f, "replies"),
            Self::Quotes => write!(f, "quotes"),
        }
    }
}

impl Default for NotificationType {
    fn default() -> NotificationType {
        Self::Follows
    }
}
