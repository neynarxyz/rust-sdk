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
pub struct AppHostGetEventResponse {
    /// Legacy event type corresponding to the requested event type: - frame_added: User adds a mini app to their account - frame_removed: User removes a mini app from their account - notifications_enabled: User enables notifications for a mini app - notifications_disabled: User disables notifications for a mini app
    #[serde(rename = "event")]
    pub event: String,
    #[serde(
        rename = "notificationDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_details: Option<Box<models::AppHostGetEventResponseNotificationDetails>>,
}

impl AppHostGetEventResponse {
    pub fn new(event: String) -> AppHostGetEventResponse {
        AppHostGetEventResponse {
            event,
            notification_details: None,
        }
    }
}
