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
pub struct WebhookPutReqBody {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<models::WebhookSubscriptionFilters>>,
    #[serde(rename = "webhook_id")]
    pub webhook_id: String,
}

impl WebhookPutReqBody {
    pub fn new(name: String, url: String, webhook_id: String) -> WebhookPutReqBody {
        WebhookPutReqBody {
            name,
            url,
            subscription: None,
            webhook_id,
        }
    }
}
