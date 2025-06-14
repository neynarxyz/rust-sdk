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
pub struct NeynarFramePage {
    /// Unique identifier for the page.
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    /// The version of the page schema.
    #[serde(rename = "version")]
    pub version: String,
    /// The title of the page.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "image")]
    pub image: Box<models::NeynarPageImage>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<models::NeynarPageButton>>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Box<models::NeynarPageInput>>,
}

impl NeynarFramePage {
    pub fn new(
        uuid: uuid::Uuid,
        version: String,
        title: String,
        image: models::NeynarPageImage,
    ) -> NeynarFramePage {
        NeynarFramePage {
            uuid,
            version,
            title,
            image: Box::new(image),
            buttons: None,
            input: None,
        }
    }
}
