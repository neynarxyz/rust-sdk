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
pub struct BanResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Option<String>>,
}

impl BanResponse {
    pub fn new(success: bool) -> BanResponse {
        BanResponse {
            success,
            message: None,
        }
    }
}
