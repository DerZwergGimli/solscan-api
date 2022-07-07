use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    pub slot: Option<i64>,
    #[serde(rename = "txHash")]
    pub tx_hash: Option<String>,
    pub fee: Option<i64>,
    pub status: Option<Status>,
    pub lamport: Option<i64>,
    pub signer: Option<Vec<String>>,
    #[serde(rename = "parsedInstruction")]
    pub parsed_instruction: Option<Vec<ParsedInstruction>>,
    #[serde(rename = "includeSPLTransfer")]
    pub include_spl_transfer: Option<bool>,
    pub err: Option<serde_json::Value>,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "loadedAddresses")]
    pub loaded_addresses: Option<LoadedAddresses>,
    #[serde(rename = "logMessages")]
    pub log_messages: Option<Vec<String>>,
    #[serde(rename = "postBalances")]
    pub post_balances: Option<Vec<i64>>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Option<Vec<TokenBalance>>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Option<Vec<i64>>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub rewards: Option<Vec<Option<serde_json::Value>>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedAddresses {
    pub readonly: Option<Vec<Option<serde_json::Value>>>,
    pub writable: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    #[serde(rename = "accountIndex")]
    pub account_index: Option<i64>,
    pub mint: Option<String>,
    pub owner: Option<String>,
    #[serde(rename = "uiTokenAmount")]
    pub ui_token_amount: Option<UiTokenAmount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiTokenAmount {
    pub amount: Option<String>,
    pub decimals: Option<i64>,
    #[serde(rename = "uiAmount")]
    pub ui_amount: Option<f64>,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInstruction {
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(rename = "type")]
    pub parsed_instruction_type: Option<String>,
    pub program: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Fail,
    Success,
}
