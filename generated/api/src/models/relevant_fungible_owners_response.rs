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
pub struct RelevantFungibleOwnersResponse {
    #[serde(rename = "top_relevant_fungible_owners_hydrated")]
    pub top_relevant_fungible_owners_hydrated: Vec<models::User>,
    #[serde(rename = "all_relevant_fungible_owners_dehydrated")]
    pub all_relevant_fungible_owners_dehydrated: Vec<models::User>,
}

impl RelevantFungibleOwnersResponse {
    pub fn new(
        top_relevant_fungible_owners_hydrated: Vec<models::User>,
        all_relevant_fungible_owners_dehydrated: Vec<models::User>,
    ) -> RelevantFungibleOwnersResponse {
        RelevantFungibleOwnersResponse {
            top_relevant_fungible_owners_hydrated,
            all_relevant_fungible_owners_dehydrated,
        }
    }
}
