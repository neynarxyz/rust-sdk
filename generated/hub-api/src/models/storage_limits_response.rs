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
pub struct StorageLimitsResponse {
    #[serde(rename = "limits")]
    pub limits: Vec<models::StorageLimit>,
}

impl StorageLimitsResponse {
    pub fn new(limits: Vec<models::StorageLimit>) -> StorageLimitsResponse {
        StorageLimitsResponse { limits }
    }
}
