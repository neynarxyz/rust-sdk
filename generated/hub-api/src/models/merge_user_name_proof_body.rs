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
pub struct MergeUserNameProofBody {
    #[serde(rename = "usernameProof", skip_serializing_if = "Option::is_none")]
    pub username_proof: Option<Box<models::UserNameProof>>,
    #[serde(
        rename = "deletedUsernameProof",
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_username_proof: Option<Box<models::UserNameProof>>,
    #[serde(
        rename = "usernameProofMessage",
        skip_serializing_if = "Option::is_none"
    )]
    pub username_proof_message: Option<Box<models::Message>>,
    #[serde(
        rename = "deletedUsernameProofMessage",
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_username_proof_message: Option<Box<models::Message>>,
}

impl MergeUserNameProofBody {
    pub fn new() -> MergeUserNameProofBody {
        MergeUserNameProofBody {
            username_proof: None,
            deleted_username_proof: None,
            username_proof_message: None,
            deleted_username_proof_message: None,
        }
    }
}
