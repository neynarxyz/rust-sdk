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
pub struct Cast {
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "parent_hash", deserialize_with = "Option::deserialize")]
    pub parent_hash: Option<String>,
    #[serde(rename = "parent_url", deserialize_with = "Option::deserialize")]
    pub parent_url: Option<String>,
    #[serde(rename = "root_parent_url", deserialize_with = "Option::deserialize")]
    pub root_parent_url: Option<String>,
    #[serde(rename = "parent_author")]
    pub parent_author: Box<models::CastParentAuthor>,
    #[serde(rename = "author")]
    pub author: Box<models::User>,
    #[serde(
        rename = "app",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<Option<Box<models::UserDehydrated>>>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "embeds")]
    pub embeds: Vec<models::Embed>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::CastNotificationType>,
}

impl Cast {
    pub fn new(
        object: Object,
        hash: String,
        parent_hash: Option<String>,
        parent_url: Option<String>,
        root_parent_url: Option<String>,
        parent_author: models::CastParentAuthor,
        author: models::User,
        text: String,
        timestamp: String,
        embeds: Vec<models::Embed>,
    ) -> Cast {
        Cast {
            object,
            hash,
            parent_hash,
            parent_url,
            root_parent_url,
            parent_author: Box::new(parent_author),
            author: Box::new(author),
            app: None,
            text,
            timestamp,
            embeds,
            r#type: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "cast")]
    Cast,
}

impl Default for Object {
    fn default() -> Object {
        Self::Cast
    }
}
