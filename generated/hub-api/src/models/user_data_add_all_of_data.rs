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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDataAddAllOfData {
    #[serde(rename = "type")]
    pub r#type: models::MessageType,
    /// The unique identifier (FID) of the user who created this message. FIDs are assigned sequentially when users register on the network and cannot be changed.
    #[serde(rename = "fid")]
    pub fid: i32,
    /// Seconds since Farcaster Epoch (2021-01-01T00:00:00Z). Used to order messages chronologically and determine the most recent state. Must be within 10 minutes of the current time when the message is created.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "network")]
    pub network: models::FarcasterNetwork,
    /// Contains the type of profile metadata being updated and its new value.
    #[serde(rename = "userDataBody")]
    pub user_data_body: Box<models::UserDataBody>,
}

impl UserDataAddAllOfData {
    pub fn new(
        r#type: models::MessageType,
        fid: i32,
        timestamp: i64,
        network: models::FarcasterNetwork,
        user_data_body: models::UserDataBody,
    ) -> UserDataAddAllOfData {
        UserDataAddAllOfData {
            r#type,
            fid,
            timestamp,
            network,
            user_data_body: Box::new(user_data_body),
        }
    }
}
