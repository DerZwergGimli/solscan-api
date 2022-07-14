use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SplTransfer {
    pub total: Option<i64>,
    pub data: Option<Vec<Datum>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub address: Option<String>,
    pub signature: Option<Vec<String>>,
    #[serde(rename = "changeType")]
    pub change_type: Option<ChangeType>,
    pub decimals: Option<i64>,
    #[serde(rename = "changeAmount")]
    pub change_amount: Option<ChangeAmount>,
    #[serde(rename = "postBalance")]
    pub post_balance: Option<String>,
    #[serde(rename = "preBalance")]
    pub pre_balance: Option<ChangeAmount>,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    pub slot: Option<i64>,
    pub fee: Option<i64>,
    pub owner: Option<String>,
    pub balance: Option<Balance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub amount: Option<String>,
    pub decimals: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeAmount {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeType {
    #[serde(rename = "dec")]
    Dec,
    #[serde(rename = "inc")]
    Inc,
}
