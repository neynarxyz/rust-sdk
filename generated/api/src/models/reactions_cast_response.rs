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
pub struct ReactionsCastResponse {
    #[serde(rename = "reactions")]
    pub reactions: Vec<models::ReactionForCast>,
    #[serde(rename = "next")]
    pub next: Box<models::NextCursor>,
}

impl ReactionsCastResponse {
    pub fn new(
        reactions: Vec<models::ReactionForCast>,
        next: models::NextCursor,
    ) -> ReactionsCastResponse {
        ReactionsCastResponse {
            reactions,
            next: Box::new(next),
        }
    }
}
