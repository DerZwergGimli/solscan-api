use log::error;
use reqwest::{Client, Error, StatusCode};
use serde::de::DeserializeOwned;

use crate::enums::solscan_endpoints::SolscanEndpoints;
use crate::enums::solscan_errors::SolscanError;
use crate::r#const::SOLSCANBASEURL;
use crate::structs::block::Block;
use crate::structs::token::Token;
use crate::structs::transaction::Transaction;

pub struct SolscanAPI {
    base_url: String,
    client: Client,
}


impl SolscanAPI {
    pub fn new() -> SolscanAPI {
        SolscanAPI {
            base_url: SOLSCANBASEURL.parse().unwrap(),
            client: Client::new(),
        }
    }
    pub fn new_with_url(solscan_url: String) -> SolscanAPI {
        SolscanAPI {
            base_url: solscan_url,
            client: Client::new(),
        }
    }

    //region private functions
    async fn fetch(&self, url_path: String) -> Result<String, SolscanError> {
        println!("{:?}", url_path);
        match {
            self.client.get(self.base_url.to_string() + url_path.as_str())
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await
        } {
            Ok(response) => {
                if response.status() == 200 {
                    match response.text().await {
                        Ok(text) => { Ok(text) }
                        Err(e) => {
                            error!("{:?}", e);
                            Err(SolscanError::APICConversionError)
                        }
                    }
                } else {
                    error!("API-Status Code is: {:?}", response.status());
                    Err(SolscanError::APIWrongStatusCode)
                }
            }
            Err(e) => {
                error!("{:?}", e);
                Err(SolscanError::APIError)
            }
        }
    }

    pub async fn solscan_fetch<T: DeserializeOwned>(&self, endpoint: SolscanEndpoints, account: Option<String>, extra_query: Option<String>, block: Option<i64>, offset: Option<i64>, limit: Option<i64>) -> Result<T, SolscanError> {
        match {
            self.fetch(
                //TODO: Make sure that all Endpoints are implemented correctly: example: /account/transaction missing beforeHash
                match endpoint {
                    SolscanEndpoints::BlockLast => endpoint.value().to_owned(),
                    SolscanEndpoints::BlockTransactions => endpoint.value().to_owned() + "?" + &*format!("block={}", block.unwrap()) + &*format!("&offset={}", offset.unwrap_or(0)) + &*format!("&limit={}", limit.unwrap_or(0)),
                    SolscanEndpoints::Block => endpoint.value().to_owned() + "/" + &*format!("{}", block.unwrap_or(0)),
                    SolscanEndpoints::TransactionLast => endpoint.value().to_owned() + "?" + &*format!("limit={}", limit.unwrap_or(10)),
                    SolscanEndpoints::Transaction => endpoint.value().to_owned() + "/" + &*account.unwrap_or("".to_string()),
                    SolscanEndpoints::AccountTokens => endpoint.value().to_owned() + "?" + &*format!("account={}", account.unwrap_or("".to_string())),
                    SolscanEndpoints::AccountTransaction => endpoint.value().to_owned() + "?" + &*format!("account={}", account.unwrap_or("".to_string())) + &*format!("&limit={}", limit.unwrap_or(10)),
                    _ => { "none".to_string() }
                }
            ).await
        } {
            Ok(api_data) => {
                match serde_json::from_str::<T>(api_data.as_str()) {
                    Ok(api_data) => {
                        Ok(api_data)
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        Err(SolscanError::SerializeError)
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
                Err(SolscanError::APIError)
            }
        }
    }
    //endregion

    //region public functions
    pub async fn ping_status(&self, endpoint: Option<String>) -> Result<StatusCode, Error> {
        Ok(self.client.get(self.base_url.to_string() + endpoint.unwrap_or_default().as_str()).header("User-Agent", "Mozilla/5.0").send().await?.status())
    }


    //region Block
    pub async fn get_block_last(&self) -> Result<Vec<Block>, SolscanError> {
        self.solscan_fetch::<Vec<Block>>(SolscanEndpoints::BlockLast, None, None, None, None, None).await
    }
    pub async fn get_block_transactions(&self, block: i64, offset: i64, limit: i64) -> Result<Vec<Block>, SolscanError> {
        self.solscan_fetch::<Vec<Block>>(SolscanEndpoints::BlockTransactions, None, None, Some(block), Some(offset), Some(limit)).await
    }
    pub async fn get_block_block(&self, block: i64) -> Result<Block, SolscanError> {
        self.solscan_fetch::<Block>(SolscanEndpoints::Block, None, None, Some(block), None, None).await
    }
    //endregion

    //region Transaction
    pub async fn get_transaction_last(&self, limit: i64) -> Result<Vec<Transaction>, SolscanError> {
        self.solscan_fetch::<Vec<Transaction>>(SolscanEndpoints::TransactionLast, None, None, None, None, Some(limit)).await
    }
    pub async fn get_transaction(&self, signature: &str) -> Result<Transaction, SolscanError> {
        self.solscan_fetch::<Transaction>(SolscanEndpoints::Transaction, Some(signature.to_string()), None, None, None, None).await
    }
    //endregion

    //region Transaction
    pub async fn get_account_tokens(&self, account: &str) -> Result<Vec<Token>, SolscanError> {
        self.solscan_fetch::<Vec<Token>>(SolscanEndpoints::AccountTokens, Some(account.to_string()), None, None, None, None).await
    }
    pub async fn get_account_transactions(&self, account: &str, before_hash: Option<String>, limit: Option<i64>) -> Result<Vec<Transaction>, SolscanError> {
        self.solscan_fetch::<Vec<Transaction>>(SolscanEndpoints::AccountTransaction, Some(account.to_string()), before_hash, None, None, limit).await
    }
    //endregion
}

#[cfg(test)]
pub mod test_solscan_inner {
    use crate::r#const::SOLSCANBASEURL;
    use crate::solscan::SolscanAPI;

    #[test]
    fn test_init_baseurl() {
        let solscan_api = SolscanAPI::new();
        assert_eq!(solscan_api.base_url, SOLSCANBASEURL);
    }
}