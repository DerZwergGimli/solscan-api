use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SolTransferList {
    pub data: Vec<Datum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "_id")]
    pub id: String,
    pub src: String,
    pub dst: String,
    pub lamport: i64,
    #[serde(rename = "blockTime")]
    pub block_time: i64,
    pub slot: i64,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub fee: i64,
    pub status: Status,
    pub decimals: i64,
    #[serde(rename = "txNumberSolTransfer")]
    pub tx_number_sol_transfer: i64,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Success,
}
