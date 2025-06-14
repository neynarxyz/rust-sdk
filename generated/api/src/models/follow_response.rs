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
pub struct FollowResponse {
    #[serde(rename = "success")]
    pub success: bool,
    /// The unique identifier of a farcaster user or app (unsigned integer)
    #[serde(rename = "target_fid")]
    pub target_fid: i32,
    #[serde(rename = "hash")]
    pub hash: String,
}

impl FollowResponse {
    pub fn new(success: bool, target_fid: i32, hash: String) -> FollowResponse {
        FollowResponse {
            success,
            target_fid,
            hash,
        }
    }
}
