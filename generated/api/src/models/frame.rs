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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum Frame {
    #[serde(rename = "vNext")]
    VNext(Box<models::FrameV1>),
    #[serde(rename = "next")]
    Next(Box<models::FrameV2>),
}

impl Default for Frame {
    fn default() -> Self {
        Self::VNext(Default::default())
    }
}
