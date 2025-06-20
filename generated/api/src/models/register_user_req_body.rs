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
pub struct RegisterUserReqBody {
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "fid")]
    pub fid: f64,
    #[serde(rename = "requested_user_custody_address")]
    pub requested_user_custody_address: String,
    #[serde(rename = "deadline")]
    pub deadline: f64,
    #[serde(rename = "fname", skip_serializing_if = "Option::is_none")]
    pub fname: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::RegisterUserReqBodyMetadata>>,
}

impl RegisterUserReqBody {
    pub fn new(
        signature: String,
        fid: f64,
        requested_user_custody_address: String,
        deadline: f64,
    ) -> RegisterUserReqBody {
        RegisterUserReqBody {
            signature,
            fid,
            requested_user_custody_address,
            deadline,
            fname: None,
            metadata: None,
        }
    }
}
