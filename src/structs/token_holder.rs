use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHolders {
    pub data: Vec<Holder>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Holder {
    pub address: String,
    pub amount: i64,
    pub decimals: i64,
    pub owner: String,
    pub rank: i64,
}
