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
pub struct FollowReqBody {
    /// UUID of the signer. `signer_uuid` is paired with API key, can't use a `uuid` made with a different API key.
    #[serde(rename = "signer_uuid")]
    pub signer_uuid: String,
    #[serde(rename = "target_fids")]
    pub target_fids: Vec<i32>,
}

impl FollowReqBody {
    pub fn new(signer_uuid: String, target_fids: Vec<i32>) -> FollowReqBody {
        FollowReqBody {
            signer_uuid,
            target_fids,
        }
    }
}
