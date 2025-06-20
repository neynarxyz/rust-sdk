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

/// AppHostPostEventBody : Request body for app host events. Can either provide a signed_message or a signer_uuid with event details.
/// Request body for app host events. Can either provide a signed_message or a signer_uuid with event details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppHostPostEventBody {
    AppHostPostEventBodyOneOf(Box<models::AppHostPostEventBodyOneOf>),
    AppHostPostEventBodyOneOf1(Box<models::AppHostPostEventBodyOneOf1>),
}

impl Default for AppHostPostEventBody {
    fn default() -> Self {
        Self::AppHostPostEventBodyOneOf(Default::default())
    }
}
