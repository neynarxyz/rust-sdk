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

/// UserDataBody : Contains the data for updating a specific field of a user's profile metadata. Each update operation modifies one profile field at a time, allowing granular control over profile information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDataBody {
    /// Specifies which profile field is being updated (e.g., profile picture, display name, bio).
    #[serde(rename = "type")]
    pub r#type: models::UserDataType,
    /// The new value for the specified profile field. The format depends on the type: URLs for profile pictures, plain text for display names and bios, etc.
    #[serde(rename = "value")]
    pub value: String,
}

impl UserDataBody {
    /// Contains the data for updating a specific field of a user's profile metadata. Each update operation modifies one profile field at a time, allowing granular control over profile information.
    pub fn new(r#type: models::UserDataType, value: String) -> UserDataBody {
        UserDataBody { r#type, value }
    }
}
