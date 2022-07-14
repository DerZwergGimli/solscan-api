//! # The Solcan Wrapper
//! This lib represent a wrapper for the SolcanAPI

use reqwest::{Client, Error, StatusCode};
use serde::de::DeserializeOwned;

use crate::enums::solscan_endpoints::SolscanEndpoints;
use crate::enums::solscan_errors::SolscanError;
use crate::r#const::SOLSCANBASEURL;
use crate::structs::account_info::AccountInfo;
use crate::structs::block::Block;
use crate::structs::chain_info::ChainInfo;
use crate::structs::sol_transfer::SolTransferList;
use crate::structs::spl_transfer::SplTransfer;
use crate::structs::token::Token;
use crate::structs::token_holder::TokenHolders;
use crate::structs::token_market_item::TokenMarketItem;
use crate::structs::token_meta::TokenMeta;
use crate::structs::transaction::Transaction;
use crate::structs::transaction_last::TransactionLast;
use crate::structs::transcation_list_item::TransactionListItem;

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
        println!("{:?}", self.base_url.to_string() + url_path.as_str());
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
                            println!("{:?}", e);
                            Err(SolscanError::APICConversionError)
                        }
                    }
                } else {
                    println!("API-Status Code is: {:?}", response.status());
                    Err(SolscanError::APIWrongStatusCode)
                }
            }
            Err(e) => {
                println!("{:?}", e);
                Err(SolscanError::APIError)
            }
        }
    }

    pub async fn solscan_fetch<T: DeserializeOwned>(&self, endpoint: SolscanEndpoints, url_endpoint: &str) -> Result<T, SolscanError> {
        match {
            self.fetch(
                match endpoint {
                    SolscanEndpoints::BlockLast => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::BlockTransactions => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::Block => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::TransactionLast => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::Transaction => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::AccountTokens => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::AccountTransaction => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::AccountStakeAccounts => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::AccountSPLTransfers => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::AccountSolTransfers => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::Account => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::TokenHolders => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::TokenMeta => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::MarketToken => endpoint.value().to_owned() + url_endpoint,
                    SolscanEndpoints::ChainInfo => endpoint.value().to_owned() + url_endpoint,
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
    pub async fn get_block_last(&self, limit: Option<i64>) -> Result<Vec<Block>, SolscanError> {
        let mut url_endpoint: String = "".to_string();
        if limit.is_some() {
            url_endpoint += &*format!("?limit={}", limit.unwrap());
        }
        self.solscan_fetch::<Vec<Block>>(SolscanEndpoints::BlockLast, url_endpoint.as_str()).await
    }
    pub async fn get_block_transactions(&self, block: i64, offset: Option<i64>, limit: Option<i64>) -> Result<Vec<Block>, SolscanError> {
        let mut url_endpoint: String = format!("?block={}", block);
        if offset.is_some() {
            url_endpoint += &*format!("&offset={}", offset.unwrap());
        }
        if limit.is_some() {
            url_endpoint += &*format!("&limit={}", limit.unwrap());
        }
        self.solscan_fetch::<Vec<Block>>(SolscanEndpoints::BlockTransactions, url_endpoint.as_str()).await
    }
    pub async fn get_block_block(&self, block: i64) -> Result<Block, SolscanError> {
        let url_endpoint: String = format!("/{}", block);
        self.solscan_fetch::<Block>(SolscanEndpoints::Block, url_endpoint.as_str()).await
    }
    //endregion

    //region Transaction
    pub async fn get_transaction_last(&self, limit: Option<i64>) -> Result<Vec<TransactionLast>, SolscanError> {
        let mut url_endpoint: String = "".to_string();
        if limit.is_some() {
            url_endpoint += &*format!("?limit={}", limit.unwrap())
        }
        self.solscan_fetch::<Vec<TransactionLast>>(SolscanEndpoints::TransactionLast, url_endpoint.as_str()).await
    }
    pub async fn get_transaction(&self, signature: &str) -> Result<Transaction, SolscanError> {
        let url_endpoint: String = format!("/{}", signature);
        self.solscan_fetch::<Transaction>(SolscanEndpoints::Transaction, url_endpoint.as_str()).await
    }
    //endregion

    //region Transaction
    pub async fn get_account_tokens(&self, account: &str) -> Result<Vec<Token>, SolscanError> {
        let url_endpoint: String = format!("?account={}", account);
        self.solscan_fetch::<Vec<Token>>(SolscanEndpoints::AccountTokens, url_endpoint.as_str()).await
    }
    pub async fn get_account_transactions(&self, account: &str, before_hash: Option<String>, limit: Option<i64>) -> Result<Vec<TransactionListItem>, SolscanError> {
        let mut url_endpoint: String = format!("?account={}", account);
        if before_hash.is_some() {
            url_endpoint += &*format!("&beforeHash={}", before_hash.unwrap())
        }
        if limit.is_some() {
            url_endpoint += &*format!("&limit={}", limit.unwrap())
        }
        self.solscan_fetch::<Vec<TransactionListItem>>(SolscanEndpoints::AccountTransaction, url_endpoint.as_str()).await
    }
    pub async fn get_account_stake_accounts(&self, account: &str) -> Result<Vec<Token>, SolscanError> {
        let url_endpoint: String = format!("?account={}", account);
        self.solscan_fetch::<Vec<Token>>(SolscanEndpoints::AccountStakeAccounts, url_endpoint.as_str()).await
    }
    pub async fn get_account_spl_transfer(&self, account: &str, form_time: Option<u64>, to_time: Option<u64>, offset: Option<i64>, limit: Option<i64>) -> Result<SplTransfer, SolscanError> {
        let mut url_endpoint: String = format!("?account={}", account);
        if form_time.is_some() {
            url_endpoint += &*format!("&form_time={}", form_time.unwrap())
        }
        if to_time.is_some() {
            url_endpoint += &*format!("&to_time={}", to_time.unwrap())
        }
        if offset.is_some() {
            url_endpoint += &*format!("&offset={}", offset.unwrap())
        }
        if limit.is_some() {
            url_endpoint += &*format!("&limit={}", limit.unwrap())
        }
        self.solscan_fetch::<SplTransfer>(SolscanEndpoints::AccountSPLTransfers, url_endpoint.as_str()).await
    }
    pub async fn get_account_sol_transfer(&self, account: &str, form_time: Option<u64>, to_time: Option<u64>, offset: Option<i64>, limit: Option<i64>) -> Result<SolTransferList, SolscanError> {
        let mut url_endpoint: String = format!("?account={}", account);
        if form_time.is_some() {
            url_endpoint += &*format!("&form_time={}", form_time.unwrap())
        }
        if to_time.is_some() {
            url_endpoint += &*format!("&to_time={}", to_time.unwrap())
        }
        if offset.is_some() {
            url_endpoint += &*format!("&offset={}", offset.unwrap())
        }
        if limit.is_some() {
            url_endpoint += &*format!("&limit={}", limit.unwrap())
        }
        self.solscan_fetch::<SolTransferList>(SolscanEndpoints::AccountSolTransfers, url_endpoint.as_str()).await
    }
    pub async fn get_account_account(&self, account: &str) -> Result<AccountInfo, SolscanError> {
        let url_endpoint: String = format!("/{}", account);
        self.solscan_fetch::<AccountInfo>(SolscanEndpoints::Account, url_endpoint.as_str()).await
    }
    //endregion

    //region Transaction
    pub async fn get_token_holders(&self, account: &str, offset: Option<i64>, limit: Option<i64>) -> Result<TokenHolders, SolscanError> {
        let mut url_endpoint: String = format!("?tokenAddress={}", account);
        if offset.is_some() {
            url_endpoint += &*format!("&offset={}", offset.unwrap())
        }
        if limit.is_some() {
            url_endpoint += &*format!("&limit={}", limit.unwrap())
        }
        self.solscan_fetch::<TokenHolders>(SolscanEndpoints::TokenHolders, url_endpoint.as_str()).await
    }
    pub async fn get_token_meta(&self, account: &str) -> Result<TokenMeta, SolscanError> {
        let url_endpoint: String = format!("?tokenAddress={}", account);
        self.solscan_fetch::<TokenMeta>(SolscanEndpoints::TokenMeta, url_endpoint.as_str()).await
    }
    //endregion

    //region MarketToken
    pub async fn get_market_token(&self, account: &str) -> Result<TokenMarketItem, SolscanError> {
        let url_endpoint: String = format!("/{}", account);
        self.solscan_fetch::<TokenMarketItem>(SolscanEndpoints::MarketToken, url_endpoint.as_str()).await
    }
    //endregion

    //region ChainInfo
    pub async fn get_chain_info(&self) -> Result<ChainInfo, SolscanError> {
        let url_endpoint: String = format!("/");
        self.solscan_fetch::<ChainInfo>(SolscanEndpoints::ChainInfo, url_endpoint.as_str()).await
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