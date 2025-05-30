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

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchUserData200Response {
    UserDataAdd(Box<models::UserDataAdd>),
    FetchUserData200ResponseOneOf(Box<models::FetchUserData200ResponseOneOf>),
}

impl Default for FetchUserData200Response {
    fn default() -> Self {
        Self::UserDataAdd(Default::default())
    }
}
