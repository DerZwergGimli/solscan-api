extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,
    pub slot: Option<i64>,
    #[serde(rename = "txHash")]
    pub tx_hash: Option<String>,
    pub fee: Option<i64>,
    pub status: Option<String>,
    pub lamport: Option<i64>,
    pub signer: Option<Vec<String>>,
    #[serde(rename = "logMessage")]
    pub log_message: Option<Vec<String>>,
    #[serde(rename = "inputAccount")]
    pub input_account: Option<Vec<InputAccount>>,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: Option<String>,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Option<Vec<InnerInstruction>>,
    #[serde(rename = "tokenBalanes")]
    pub token_balanes: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "parsedInstruction")]
    pub parsed_instruction: Option<Vec<TransactionParsedInstruction>>,
    pub confirmations: Option<i64>,
    #[serde(rename = "tokenTransfers")]
    pub token_transfers: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "solTransfers")]
    pub sol_transfers: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "serumTransactions")]
    pub serum_transactions: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "raydiumTransactions")]
    pub raydium_transactions: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "unknownTransfers")]
    pub unknown_transfers: Option<Vec<UnknownTransfer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerInstruction {
    pub index: Option<i64>,
    #[serde(rename = "parsedInstructions")]
    pub parsed_instructions: Option<Vec<InnerInstructionParsedInstruction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerInstructionParsedInstruction {
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    pub program: Option<String>,
    #[serde(rename = "type")]
    pub parsed_instruction_type: Option<String>,
    pub name: Option<String>,
    pub params: Option<PurpleParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleParams {
    pub source: Option<String>,
    pub destination: Option<String>,
    pub amount: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputAccount {
    pub account: Option<String>,
    pub signer: Option<bool>,
    pub writable: Option<bool>,
    #[serde(rename = "preBalance")]
    pub pre_balance: Option<i64>,
    #[serde(rename = "postBalance")]
    pub post_balance: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionParsedInstruction {
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(rename = "type")]
    pub parsed_instruction_type: Option<String>,
    pub data: Option<String>,
    #[serde(rename = "dataEncode")]
    pub data_encode: Option<String>,
    pub name: Option<String>,
    pub params: Option<FluffyParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyParams {
    #[serde(rename = "Account0")]
    pub account0: Option<String>,
    #[serde(rename = "Account1")]
    pub account1: Option<String>,
    #[serde(rename = "Account2")]
    pub account2: Option<String>,
    #[serde(rename = "Account3")]
    pub account3: Option<String>,
    #[serde(rename = "Account4")]
    pub account4: Option<String>,
    #[serde(rename = "Account5")]
    pub account5: Option<String>,
    #[serde(rename = "Account6")]
    pub account6: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnknownTransfer {
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    pub event: Option<Vec<Event>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub source: Option<String>,
    pub destination: Option<String>,
    pub amount: Option<i64>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub decimals: Option<i64>,
    pub symbol: Option<String>,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
}
