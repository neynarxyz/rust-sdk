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
pub struct TransactionSendFungiblesResponse {
    #[serde(rename = "send_receipts")]
    pub send_receipts: Vec<models::TransactionSendFungiblesReceipt>,
    #[serde(rename = "transactions")]
    pub transactions: Vec<models::TransactionSendTxInfo>,
}

impl TransactionSendFungiblesResponse {
    pub fn new(
        send_receipts: Vec<models::TransactionSendFungiblesReceipt>,
        transactions: Vec<models::TransactionSendTxInfo>,
    ) -> TransactionSendFungiblesResponse {
        TransactionSendFungiblesResponse {
            send_receipts,
            transactions,
        }
    }
}
