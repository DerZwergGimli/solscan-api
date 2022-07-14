use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMeta {
    pub symbol: String,
    pub address: String,
    pub name: String,
    pub icon: String,
    pub website: String,
    pub twitter: String,
    pub decimals: i64,
    #[serde(rename = "coingeckoId")]
    pub coingecko_id: String,
    pub price: f64,
    pub volume: i64,
    #[serde(rename = "tokenAuthority")]
    pub token_authority: Option<serde_json::Value>,
    pub supply: String,
    #[serde(rename = "type")]
    pub token_meta_type: String,
}
