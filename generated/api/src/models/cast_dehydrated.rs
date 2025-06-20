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
pub struct CastDehydrated {
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::UserDehydrated>>,
    #[serde(
        rename = "app",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<Option<Box<models::UserDehydrated>>>,
}

impl CastDehydrated {
    pub fn new(object: Object, hash: String) -> CastDehydrated {
        CastDehydrated {
            object,
            hash,
            author: None,
            app: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "cast_dehydrated")]
    CastDehydrated,
}

impl Default for Object {
    fn default() -> Object {
        Self::CastDehydrated
    }
}
