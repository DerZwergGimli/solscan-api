use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub meta: Meta,
    pub transaction: TransactionClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub err: Option<serde_json::Value>,
    pub fee: i64,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Vec<Option<serde_json::Value>>,
    #[serde(rename = "loadedAddresses")]
    pub loaded_addresses: LoadedAddresses,
    #[serde(rename = "logMessages")]
    pub log_messages: Vec<String>,
    #[serde(rename = "postBalances")]
    pub post_balances: Vec<i64>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Vec<Option<serde_json::Value>>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Vec<i64>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Vec<Option<serde_json::Value>>,
    pub rewards: Vec<Option<serde_json::Value>>,
    pub status: Status,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedAddresses {
    pub readonly: Vec<Option<serde_json::Value>>,
    pub writable: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "Ok")]
    pub ok: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionClass {
    pub message: Message,
    pub signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "accountKeys")]
    pub account_keys: Vec<AccountKey>,
    #[serde(rename = "addressTableLookups")]
    pub address_table_lookups: Option<serde_json::Value>,
    pub instructions: Vec<Instruction>,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountKey {
    pub pubkey: String,
    pub signer: bool,
    pub writable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub accounts: Option<Vec<String>>,
    pub data: Option<String>,
    #[serde(rename = "programId")]
    pub program_id: String,
    pub parsed: Option<Parsed>,
    pub program: Option<Program>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
    pub info: Info,
    #[serde(rename = "type")]
    pub parsed_type: Program,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "clockSysvar")]
    pub clock_sysvar: String,
    #[serde(rename = "slotHashesSysvar")]
    pub slot_hashes_sysvar: String,
    pub vote: Vote,
    #[serde(rename = "voteAccount")]
    pub vote_account: String,
    #[serde(rename = "voteAuthority")]
    pub vote_authority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub hash: String,
    pub slots: Vec<i64>,
    pub timestamp: Option<i64>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Program {
    #[serde(rename = "vote")]
    Vote,
}



