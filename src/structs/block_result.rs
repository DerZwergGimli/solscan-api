use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResult {
    #[serde(rename = "currentSlot")]
    pub current_slot: Option<i64>,
    pub result: Option<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockHeight")]
    pub block_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentSlot")]
    pub parent_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previousBlockhash")]
    pub previous_blockhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "feeRewards")]
    pub fee_rewards: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transactionCount")]
    pub transaction_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
