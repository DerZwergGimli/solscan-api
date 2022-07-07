use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
    #[serde(rename = "tokenAmount")]
    pub token_amount: Option<TokenAmount>,
    #[serde(rename = "tokenAccount")]
    pub token_account: Option<String>,
    #[serde(rename = "tokenName")]
    pub token_name: Option<String>,
    #[serde(rename = "tokenIcon")]
    pub token_icon: Option<String>,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: Option<i64>,
    pub lamports: Option<i64>,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    pub amount: Option<String>,
    pub decimals: Option<i64>,
    #[serde(rename = "uiAmount")]
    pub ui_amount: Option<f64>,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: Option<String>,
}
