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
pub struct CastsMetrics {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "resolution_in_seconds")]
    pub resolution_in_seconds: i32,
    #[serde(rename = "cast_count")]
    pub cast_count: i32,
}

impl CastsMetrics {
    pub fn new(start: String, resolution_in_seconds: i32, cast_count: i32) -> CastsMetrics {
        CastsMetrics {
            start,
            resolution_in_seconds,
            cast_count,
        }
    }
}
