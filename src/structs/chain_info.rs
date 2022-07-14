use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainInfo {
    #[serde(rename = "blockHeight")]
    pub block_height: i64,
    #[serde(rename = "currentEpoch")]
    pub current_epoch: i64,
    #[serde(rename = "absoluteSlot")]
    pub absolute_slot: i64,
    #[serde(rename = "transactionCount")]
    pub transaction_count: i64,
}

