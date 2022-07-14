use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTransaction {
    pub meta: Option<Meta>,
    pub transaction: Option<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub err: Option<Err>,
    pub fee: Option<i64>,
    #[serde(rename = "innerInstructions")]
    pub inner_instructions: Option<Vec<InnerInstruction>>,
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
    pub status: Option<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Err {
    #[serde(rename = "InstructionError")]
    pub instruction_error: Option<Vec<InstructionErrorElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionErrorClass {
    #[serde(rename = "Custom")]
    pub custom: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerInstruction {
    pub index: Option<i64>,
    pub instructions: Option<Vec<InnerInstructionInstruction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerInstructionInstruction {
    pub parsed: Option<PurpleParsed>,
    pub program: Option<Program>,
    #[serde(rename = "programId")]
    pub program_id: Option<ProgramId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleParsed {
    pub info: Option<PurpleInfo>,
    #[serde(rename = "type")]
    pub parsed_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleInfo {
    pub amount: Option<String>,
    pub authority: Option<String>,
    pub destination: Option<String>,
    pub source: Option<String>,
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
    #[serde(rename = "programId")]
    pub program_id: Option<ProgramId>,
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
pub struct Status {
    #[serde(rename = "Ok")]
    pub ok: Option<serde_json::Value>,
    #[serde(rename = "Err")]
    pub err: Option<Err>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub message: Option<Message>,
    pub signatures: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "accountKeys")]
    pub account_keys: Option<Vec<AccountKey>>,
    #[serde(rename = "addressTableLookups")]
    pub address_table_lookups: Option<serde_json::Value>,
    pub instructions: Option<Vec<MessageInstruction>>,
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
pub struct MessageInstruction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<FluffyParsed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<Program>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "programId")]
    pub program_id: Option<ProgramId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyParsed {
    pub info: Option<FluffyInfo>,
    #[serde(rename = "type")]
    pub parsed_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyInfo {
    #[serde(rename = "clockSysvar")]
    pub clock_sysvar: Option<ClockSysvar>,
    #[serde(rename = "slotHashesSysvar")]
    pub slot_hashes_sysvar: Option<SlotHashesSysvar>,
    pub vote: Option<Vote>,
    #[serde(rename = "voteAccount")]
    pub vote_account: Option<String>,
    #[serde(rename = "voteAuthority")]
    pub vote_authority: Option<String>,
    pub amount: Option<String>,
    pub authority: Option<String>,
    pub destination: Option<String>,
    pub source: Option<String>,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "vote")]
    Vote,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Program {
    #[serde(rename = "spl-token")]
    SplToken,
    #[serde(rename = "vote")]
    Vote,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProgramId {
    ComputeBudget111111111111111111111111111111,
    #[serde(rename = "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH")]
    FsJ3A3U2Vn5CTVofAjvy6Y5KwAbjAqYWpe4975Bi2EpH,
    #[serde(rename = "sarbL3oPVviJSsRzZYzPCMA7p7pFGzRBFQFU62xNpNz")]
    SarbL3OPVviJSsRzZYzPcma7P7PFGzRbfqfu62XNpNz,
    #[serde(rename = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")]
    The9XQeWvG816BUx9EPjHmaT23YvVm2ZWbrrpZb9PusVFin,
    #[serde(rename = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    TokenkegQfeZyiNwAJbNbGkpfxcWuBvf9Ss623Vq5Da,
    Vote111111111111111111111111111111111111111,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClockSysvar {
    #[serde(rename = "SysvarC1ock11111111111111111111111111111111")]
    SysvarC1Ock11111111111111111111111111111111,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SlotHashesSysvar {
    #[serde(rename = "SysvarS1otHashes111111111111111111111111111")]
    SysvarS1OtHashes111111111111111111111111111,
}
