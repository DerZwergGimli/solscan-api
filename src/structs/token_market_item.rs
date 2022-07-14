use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMarketItem {
    #[serde(rename = "priceUsdt")]
    pub price_usdt: f64,
    #[serde(rename = "volumeUsdt")]
    pub volume_usdt: i64,
    #[serde(rename = "marketCapFD")]
    pub market_cap_fd: Option<i64>,
    #[serde(rename = "marketCapRank")]
    pub market_cap_rank: Option<i64>,
    #[serde(rename = "priceChange24h")]
    pub price_change24_h: Option<f64>,
}

