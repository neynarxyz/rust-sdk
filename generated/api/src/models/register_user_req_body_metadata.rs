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
pub struct RegisterUserReqBodyMetadata {
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "pfp_url", skip_serializing_if = "Option::is_none")]
    pub pfp_url: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "verified_accounts", skip_serializing_if = "Option::is_none")]
    pub verified_accounts: Option<Box<models::RegisterUserReqBodyMetadataVerifiedAccounts>>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::RegisterUserReqBodyMetadataLocation>>,
}

impl RegisterUserReqBodyMetadata {
    pub fn new() -> RegisterUserReqBodyMetadata {
        RegisterUserReqBodyMetadata {
            bio: None,
            pfp_url: None,
            url: None,
            username: None,
            display_name: None,
            verified_accounts: None,
            location: None,
        }
    }
}
