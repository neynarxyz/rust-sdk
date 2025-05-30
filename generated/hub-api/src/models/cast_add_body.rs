/*
 * Farcaster Hub API
 *
 * Perform basic queries of Farcaster state via the REST API of a Farcaster hub. See the [Neynar docs](https://docs.neynar.com/reference) for more details.
 *
 * The version of the OpenAPI document: 2.35.0
 * Contact: team@neynar.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CastAddBody : Adds a new Cast
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CastAddBody {
    #[serde(rename = "embedsDeprecated")]
    pub embeds_deprecated: Vec<String>,
    #[serde(rename = "mentions")]
    pub mentions: Vec<i32>,
    #[serde(rename = "parentCastId", skip_serializing_if = "Option::is_none")]
    pub parent_cast_id: Option<Box<models::CastId>>,
    #[serde(rename = "parentUrl", skip_serializing_if = "Option::is_none")]
    pub parent_url: Option<String>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "mentionsPositions")]
    pub mentions_positions: Vec<i64>,
    #[serde(rename = "embeds")]
    pub embeds: Vec<models::Embed>,
}

impl CastAddBody {
    /// Adds a new Cast
    pub fn new(
        embeds_deprecated: Vec<String>,
        mentions: Vec<i32>,
        text: String,
        mentions_positions: Vec<i64>,
        embeds: Vec<models::Embed>,
    ) -> CastAddBody {
        CastAddBody {
            embeds_deprecated,
            mentions,
            parent_cast_id: None,
            parent_url: None,
            text,
            mentions_positions,
            embeds,
        }
    }
}
