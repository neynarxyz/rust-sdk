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

/// CastParamType : The query param accepted by the API. Sent along with identifier param. url - Cast identifier is a url hash - Cast identifier is a hash
/// The query param accepted by the API. Sent along with identifier param. url - Cast identifier is a url hash - Cast identifier is a hash
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CastParamType {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "hash")]
    Hash,
}

impl std::fmt::Display for CastParamType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Url => write!(f, "url"),
            Self::Hash => write!(f, "hash"),
        }
    }
}

impl Default for CastParamType {
    fn default() -> CastParamType {
        Self::Url
    }
}
