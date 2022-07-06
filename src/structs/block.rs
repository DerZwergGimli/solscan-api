use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "currentSlot")]
    pub current_slot: i64,
    pub result: Result,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "blockHeight")]
    pub block_height: Option<i64>,
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    pub blockhash: Option<String>,
    #[serde(rename = "parentSlot")]
    pub parent_slot: Option<i64>,
    #[serde(rename = "previousBlockhash")]
    pub previous_blockhash: Option<String>,
    #[serde(rename = "feeRewards")]
    pub fee_rewards: Option<i64>,
    pub validator: Option<String>,
    #[serde(rename = "transactionCount")]
    pub transaction_count: Option<i64>,
    pub code: Option<i64>,
    pub message: Option<String>,
}