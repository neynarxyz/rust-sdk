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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignerEventType {
    #[serde(rename = "SIGNER_EVENT_TYPE_ADD")]
    SignerEventTypeAdd,
    #[serde(rename = "SIGNER_EVENT_TYPE_REMOVE")]
    SignerEventTypeRemove,
    #[serde(rename = "SIGNER_EVENT_TYPE_ADMIN_RESET")]
    SignerEventTypeAdminReset,
}

impl std::fmt::Display for SignerEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SignerEventTypeAdd => write!(f, "SIGNER_EVENT_TYPE_ADD"),
            Self::SignerEventTypeRemove => write!(f, "SIGNER_EVENT_TYPE_REMOVE"),
            Self::SignerEventTypeAdminReset => write!(f, "SIGNER_EVENT_TYPE_ADMIN_RESET"),
        }
    }
}

impl Default for SignerEventType {
    fn default() -> SignerEventType {
        Self::SignerEventTypeAdd
    }
}
