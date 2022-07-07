use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub lamports: i64,
    #[serde(rename = "ownerProgram")]
    pub owner_program: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: i64,
    pub executable: bool,
    pub account: String,
    #[serde(rename = "tokenInfo")]
    pub token_info: TokenInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub volume: i64,
    pub decimals: i64,
    #[serde(rename = "tokenAuthority")]
    pub token_authority: Option<serde_json::Value>,
    pub supply: String,
    #[serde(rename = "type")]
    pub token_info_type: String,
}
