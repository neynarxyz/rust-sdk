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

/// UserViewerContext : Adds context on the viewer's follow relationship with the user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserViewerContext {
    /// Indicates if the viewer is following the user.
    #[serde(rename = "following")]
    pub following: bool,
    /// Indicates if the viewer is followed by the user.
    #[serde(rename = "followed_by")]
    pub followed_by: bool,
    /// Indicates if the viewer is blocking the user.
    #[serde(rename = "blocking")]
    pub blocking: bool,
    /// Indicates if the viewer is blocked by the user.
    #[serde(rename = "blocked_by")]
    pub blocked_by: bool,
}

impl UserViewerContext {
    /// Adds context on the viewer's follow relationship with the user.
    pub fn new(
        following: bool,
        followed_by: bool,
        blocking: bool,
        blocked_by: bool,
    ) -> UserViewerContext {
        UserViewerContext {
            following,
            followed_by,
            blocking,
            blocked_by,
        }
    }
}
