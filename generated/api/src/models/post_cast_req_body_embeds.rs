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
#[serde(untagged)]
pub enum PostCastReqBodyEmbeds {
    PostCastReqBodyEmbedsOneOf(Box<models::PostCastReqBodyEmbedsOneOf>),
    PostCastReqBodyEmbedsOneOf1(Box<models::PostCastReqBodyEmbedsOneOf1>),
    PostCastReqBodyEmbedsOneOf2(Box<models::PostCastReqBodyEmbedsOneOf2>),
}

impl Default for PostCastReqBodyEmbeds {
    fn default() -> Self {
        Self::PostCastReqBodyEmbedsOneOf(Default::default())
    }
}
