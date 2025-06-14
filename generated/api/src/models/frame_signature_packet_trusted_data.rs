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

/// FrameSignaturePacketTrustedData : Trusted data from the user
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameSignaturePacketTrustedData {
    /// Message bytes
    #[serde(rename = "messageBytes", skip_serializing_if = "Option::is_none")]
    pub message_bytes: Option<String>,
}

impl FrameSignaturePacketTrustedData {
    /// Trusted data from the user
    pub fn new() -> FrameSignaturePacketTrustedData {
        FrameSignaturePacketTrustedData {
            message_bytes: None,
        }
    }
}
