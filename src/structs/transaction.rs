use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub parsed_instructions: Option<Vec<TransactionParsedInstruction>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub parsed_instruction_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataEncode")]
    pub data_encode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Extra>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "associatedAccount")]
    pub associated_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokenProgramId")]
    pub token_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newAccount")]
    pub new_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transferAmount(SOL)")]
    pub transfer_amount_sol: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "programOwner")]
    pub program_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extra {
    pub source: Option<String>,
    pub destination: Option<String>,
    pub authority: Option<String>,
    pub amount: Option<String>,
    pub mint: Option<String>,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
    pub decimals: Option<i64>,
    #[serde(rename = "sourceOwner")]
    pub source_owner: Option<String>,
    #[serde(rename = "destinationOwner")]
    pub destination_owner: Option<String>,
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
