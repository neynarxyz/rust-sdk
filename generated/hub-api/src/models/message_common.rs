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
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCommon {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "hashScheme")]
    pub hash_scheme: models::HashScheme,
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "signature")]
    pub signature: Vec<u8>,
    #[serde(rename = "signatureScheme")]
    pub signature_scheme: models::SignatureScheme,
    #[serde(rename = "signer")]
    pub signer: String,
}

impl MessageCommon {
    pub fn new(
        hash: String,
        hash_scheme: models::HashScheme,
        signature: Vec<u8>,
        signature_scheme: models::SignatureScheme,
        signer: String,
    ) -> MessageCommon {
        MessageCommon {
            hash,
            hash_scheme,
            signature,
            signature_scheme,
            signer,
        }
    }
}
