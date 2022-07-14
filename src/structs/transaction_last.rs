extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_optional_field<'de, T, D>(deserializer: D)
                                         -> Result<Option<Option<T>>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de>
{
    Ok(Some(Option::deserialize(deserializer)?))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionLast {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub err: Option<Err>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Option<Vec<Option<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loadedAddresses")]
    pub loaded_addresses: Option<LoadedAddresses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logMessages")]
    pub log_messages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "postBalances")]
    pub post_balances: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Option<Vec<TokenBalance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preBalances")]
    pub pre_balances: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<Option<serde_json::Value>>>,
    pub status: Option<Status>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Err {
    #[serde(rename = "InstructionError")]
    pub instruction_error: Option<Vec<InstructionErrorElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionErrorClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Custom")]
    pub custom: Option<i64>,
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

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Status {
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Ok")]
    pub ok: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Err")]
    pub err: Option<Err>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "accountKeys")]
    pub account_keys: Option<Vec<AccountKey>>,
    #[serde(rename = "addressTableLookups")]
    pub address_table_lookups: Option<serde_json::Value>,
    pub instructions: Option<Vec<Instruction>>,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountKey {
    pub pubkey: Option<String>,
    pub signer: Option<bool>,
    pub writable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<Parsed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub parsed_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lamports: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clockSysvar")]
    pub clock_sysvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "slotHashesSysvar")]
    pub slot_hashes_sysvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<Vote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voteAccount")]
    pub vote_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voteAuthority")]
    pub vote_authority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub hash: Option<String>,
    pub slots: Option<Vec<i64>>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InstructionErrorElement {
    InstructionErrorClass(InstructionErrorClass),
    Integer(i64),
    String(String),
}
