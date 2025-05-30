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

/// UserDataType : Specifies which field of a user's profile is being updated. - USER_DATA_TYPE_PFP: Profile Picture URL for the user's avatar - USER_DATA_TYPE_DISPLAY: Display Name shown on the user's profile - USER_DATA_TYPE_BIO: Biography or description of the user - USER_DATA_TYPE_URL: Website or social media link for the user - USER_DATA_TYPE_USERNAME: Preferred username for the user
/// Specifies which field of a user's profile is being updated. - USER_DATA_TYPE_PFP: Profile Picture URL for the user's avatar - USER_DATA_TYPE_DISPLAY: Display Name shown on the user's profile - USER_DATA_TYPE_BIO: Biography or description of the user - USER_DATA_TYPE_URL: Website or social media link for the user - USER_DATA_TYPE_USERNAME: Preferred username for the user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserDataType {
    #[serde(rename = "USER_DATA_TYPE_PFP")]
    UserDataTypePfp,
    #[serde(rename = "USER_DATA_TYPE_DISPLAY")]
    UserDataTypeDisplay,
    #[serde(rename = "USER_DATA_TYPE_BIO")]
    UserDataTypeBio,
    #[serde(rename = "USER_DATA_TYPE_URL")]
    UserDataTypeUrl,
    #[serde(rename = "USER_DATA_TYPE_USERNAME")]
    UserDataTypeUsername,
}

impl std::fmt::Display for UserDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UserDataTypePfp => write!(f, "USER_DATA_TYPE_PFP"),
            Self::UserDataTypeDisplay => write!(f, "USER_DATA_TYPE_DISPLAY"),
            Self::UserDataTypeBio => write!(f, "USER_DATA_TYPE_BIO"),
            Self::UserDataTypeUrl => write!(f, "USER_DATA_TYPE_URL"),
            Self::UserDataTypeUsername => write!(f, "USER_DATA_TYPE_USERNAME"),
        }
    }
}

impl Default for UserDataType {
    fn default() -> UserDataType {
        Self::UserDataTypePfp
    }
}
