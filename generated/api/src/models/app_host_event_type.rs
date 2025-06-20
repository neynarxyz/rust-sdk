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

/// AppHostEventType : Types of events that can occur between a user and an app host: - frame_added: User adds a mini app to their account - frame_removed: User removes a mini app from their account - notifications_enabled: User enables notifications for a mini app - notifications_disabled: User disables notifications for a mini app
/// Types of events that can occur between a user and an app host: - frame_added: User adds a mini app to their account - frame_removed: User removes a mini app from their account - notifications_enabled: User enables notifications for a mini app - notifications_disabled: User disables notifications for a mini app
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppHostEventType {
    #[serde(rename = "frame_added")]
    FrameAdded,
    #[serde(rename = "frame_removed")]
    FrameRemoved,
    #[serde(rename = "notifications_enabled")]
    NotificationsEnabled,
    #[serde(rename = "notifications_disabled")]
    NotificationsDisabled,
}

impl std::fmt::Display for AppHostEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FrameAdded => write!(f, "frame_added"),
            Self::FrameRemoved => write!(f, "frame_removed"),
            Self::NotificationsEnabled => write!(f, "notifications_enabled"),
            Self::NotificationsDisabled => write!(f, "notifications_disabled"),
        }
    }
}

impl Default for AppHostEventType {
    fn default() -> AppHostEventType {
        Self::FrameAdded
    }
}
