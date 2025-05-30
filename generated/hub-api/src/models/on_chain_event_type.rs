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
pub enum OnChainEventType {
    #[serde(rename = "EVENT_TYPE_SIGNER")]
    EventTypeSigner,
    #[serde(rename = "EVENT_TYPE_SIGNER_MIGRATED")]
    EventTypeSignerMigrated,
    #[serde(rename = "EVENT_TYPE_ID_REGISTER")]
    EventTypeIdRegister,
    #[serde(rename = "EVENT_TYPE_STORAGE_RENT")]
    EventTypeStorageRent,
}

impl std::fmt::Display for OnChainEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EventTypeSigner => write!(f, "EVENT_TYPE_SIGNER"),
            Self::EventTypeSignerMigrated => write!(f, "EVENT_TYPE_SIGNER_MIGRATED"),
            Self::EventTypeIdRegister => write!(f, "EVENT_TYPE_ID_REGISTER"),
            Self::EventTypeStorageRent => write!(f, "EVENT_TYPE_STORAGE_RENT"),
        }
    }
}

impl Default for OnChainEventType {
    fn default() -> OnChainEventType {
        Self::EventTypeSigner
    }
}
