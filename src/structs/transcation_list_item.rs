use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionListItem {
    #[serde(rename = "blockTime")]
    pub block_time: i64,
    pub slot: i64,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub fee: i64,
    pub status: Status,
    pub lamport: i64,
    pub signer: Vec<String>,
    #[serde(rename = "parsedInstruction")]
    pub parsed_instruction: Vec<ParsedInstruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInstruction {
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "type")]
    pub parsed_instruction_type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "createAssociatedAccount")]
    CreateAssociatedAccount,
    Unknown,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Success,
}
