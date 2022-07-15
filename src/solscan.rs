//! # The Solscan Wrapper
//! This represent a wrapper for the SolscanAPI

use reqwest::{Client, Error, StatusCode};
use serde::de::DeserializeOwned;

use crate::enums::solscan_endpoints::SolscanEndpoints;
use crate::enums::solscan_errors::SolscanError;
use crate::r#const::SOLSCANBASEURL;
use crate::structs::account_info::AccountInfo;
use crate::structs::block_result::BlockResult;
use crate::structs::chain_info::ChainInfo;
use crate::structs::sol_transfer::SolTransferList;
use crate::structs::spl_transfer::SplTransfer;
use crate::structs::token::Token;
use crate::structs::token_holder::TokenHolders;
use crate::structs::token_market_item::TokenMarketItem;
use crate::structs::token_meta::TokenMeta;
use crate::structs::transaction::Transaction;
use crate::structs::transaction_last::TransactionLast;
use crate::structs::transaction_list_item::TransactionListItem;

/// `SolscanAPI` is a struct that contains a `String` for the SolscanURL and a `Client` which is the instance.
///
/// Properties:
///
/// A doc comment.
/// A doc comment.
/// A doc comment.
/// * `base_url`: The base URL for the Solscan API.
/// * `client`: This is the HTTP client that will be used to make requests to the Solscan API.
pub struct SolscanAPI {
    base_url: String,
    client: Client,
}


/// Creating a default implementation for the SolscanAPI struct.
impl Default for SolscanAPI {
    /// It creates a new instance of the SolscanAPI struct.
    ///
    /// Returns:
    ///
    /// A new instance of the SolscanAPI struct.
    fn default() -> Self {
        SolscanAPI::new()
    }
}


/// Implementation of the struct called SolscanAPI.
impl SolscanAPI {
    /// It creates a new instance of the SolscanAPI struct.
    ///
    /// Returns:
    ///
    /// A new instance of the SolscanAPI struct.
    pub fn new() -> SolscanAPI {
        SolscanAPI {
            base_url: SOLSCANBASEURL.parse().unwrap(),
            client: Client::new(),
        }
    }
    /// > This function creates a new instance of the SolscanAPI struct, which is used to make
    /// custom requests to the Solscan API (mostly useful for mockups)
    ///
    /// Arguments:
    ///
    /// * `solscan_url`: The URL of the Solscan API.
    ///
    /// Returns:
    ///
    /// A new instance of the SolscanAPI struct.
    pub fn new_with_url(solscan_url: String) -> SolscanAPI {
        SolscanAPI {
            base_url: solscan_url,
            client: Client::new(),
        }
    }

    //region private functions
    /// It takes a string, appends it to the base url, makes a get request, checks the status code, and
    /// returns the text of the response.
    /// Represents the actual GET request.
    ///
    /// Arguments:
    ///
    /// * `url_path`: The path to the API endpoint you want to call.
    ///
    /// Returns:
    ///
    /// A Result<String, SolscanError>
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

    /// It chooses which endpoint to use to fetch data from the Solscan API.
    ///
    /// Arguments:
    ///
    /// * `endpoint`: SolscanEndpoints - This is the endpoint you want to use.
    /// * `url_endpoint`: The endpoint to be appended to the base url.
    ///
    /// Returns:
    ///
    /// A Result<T, SolscanError>
    async fn solscan_fetch<T: DeserializeOwned>(&self, endpoint: SolscanEndpoints, url_endpoint: &str) -> Result<T, SolscanError> {
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

    //region Ping
    /// It checks the status of the endpoint.
    ///
    /// Arguments:
    ///
    /// * `endpoint`: The endpoint to ping. If none is provided, the base url will be used.
    ///
    /// Returns:
    ///
    /// A Result<StatusCode, Error>
    pub async fn ping_status(&self, endpoint: Option<String>) -> Result<StatusCode, Error> {
        Ok(self.client.get(self.base_url.to_string() + endpoint.unwrap_or_default().as_str()).header("User-Agent", "Mozilla/5.0").send().await?.status())
    }
    //endregion

    //region Block
    /// It gets the last block
    ///
    /// Arguments:
    ///
    /// * `limit`: The number of blocks to return.
    ///
    /// Returns:
    ///
    /// A Result<Vec<BlockResult>, SolscanError>
    pub async fn get_block_last(&self, limit: Option<i64>) -> Result<Vec<BlockResult>, SolscanError> {
        let mut url_endpoint: String = "".to_string();
        if limit.is_some() {
            url_endpoint += &*format!("?limit={}", limit.unwrap());
        }
        self.solscan_fetch::<Vec<BlockResult>>(SolscanEndpoints::BlockLast, url_endpoint.as_str()).await
    }
    /// It gets the transactions for a block.
    ///
    /// Arguments:
    ///
    /// * `block`: The block number to get transactions for.
    /// * `offset`: The offset of the first transaction to return.
    /// * `limit`: The number of transactions to return.
    ///
    /// Returns:
    ///
    /// A Result<Vec<TransactionLast>, SolscanError>
    pub async fn get_block_transactions(&self, block: i64, offset: Option<i64>, limit: Option<i64>) -> Result<Vec<TransactionLast>, SolscanError> {
        let mut url_endpoint: String = format!("?block={}", block);
        if offset.is_some() {
            url_endpoint += &*format!("&offset={}", offset.unwrap());
        }
        if limit.is_some() {
            url_endpoint += &*format!("&limit={}", limit.unwrap());
        }
        self.solscan_fetch::<Vec<TransactionLast>>(SolscanEndpoints::BlockTransactions, url_endpoint.as_str()).await
    }
    /// It gets the block information for a given block number.
    ///
    /// Arguments:
    ///
    /// * `block`: The block number you want to query
    ///
    /// Returns:
    ///
    /// A Result<BlockResult, SolscanError>
    pub async fn get_block_block(&self, block: i64) -> Result<BlockResult, SolscanError> {
        let url_endpoint: String = format!("/{}", block);
        self.solscan_fetch::<BlockResult>(SolscanEndpoints::Block, url_endpoint.as_str()).await
    }
    //endregion

    //region Transaction
    /// It gets the last transactions from the blockchain.
    ///
    /// Arguments:
    ///
    /// * `limit`: The number of transactions to return.
    ///
    /// Returns:
    ///
    /// A  Result<Vec<TransactionLast>, SolscanError>
    pub async fn get_transaction_last(&self, limit: Option<i64>) -> Result<Vec<TransactionLast>, SolscanError> {
        let mut url_endpoint: String = "".to_string();
        if limit.is_some() {
            url_endpoint += &*format!("?limit={}", limit.unwrap())
        }
        self.solscan_fetch::<Vec<TransactionLast>>(SolscanEndpoints::TransactionLast, url_endpoint.as_str()).await
    }
    /// It fetches a transaction from the blockchain.
    ///
    /// Arguments:
    ///
    /// * `signature`: The transaction hash
    ///
    /// Returns:
    ///
    /// A Result<Transaction, SolscanError>
    pub async fn get_transaction(&self, signature: &str) -> Result<Transaction, SolscanError> {
        let url_endpoint: String = format!("/{}", signature);
        self.solscan_fetch::<Transaction>(SolscanEndpoints::Transaction, url_endpoint.as_str()).await
    }
    //endregion

    //region Transaction
    /// It fetches the tokens associated with an account.
    ///
    /// Arguments:
    ///
    /// * `account`: The address of the account you want to get the tokens for.
    ///
    /// Returns:
    ///
    /// A Result<Vec<Token>, SolscanError>
    pub async fn get_account_tokens(&self, account: &str) -> Result<Vec<Token>, SolscanError> {
        let url_endpoint: String = format!("?account={}", account);
        self.solscan_fetch::<Vec<Token>>(SolscanEndpoints::AccountTokens, url_endpoint.as_str()).await
    }
    /// It gets the transactions for a given account.
    ///
    /// Arguments:
    ///
    /// * `account`: The address of the account you want to get the transactions for.
    /// * `before_hash`: The hash of the transaction you want to start from.
    /// * `limit`: The number of transactions to return.
    ///
    /// Returns:
    ///
    /// A Result<Vec<TransactionListItem>, SolscanError>
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
    /// It returns a list of accounts that have staked to the given account.
    ///
    /// Arguments:
    ///
    /// * `account`: The account address to query
    ///
    /// Returns:
    ///
    /// A Result<Vec<Token>, SolscanError>
    pub async fn get_account_stake_accounts(&self, account: &str) -> Result<Vec<Token>, SolscanError> {
        let url_endpoint: String = format!("?account={}", account);
        self.solscan_fetch::<Vec<Token>>(SolscanEndpoints::AccountStakeAccounts, url_endpoint.as_str()).await
    }
    /// It fetches the account spl transfer data from the solscan api.
    ///
    /// Arguments:
    ///
    /// * `account`: The account address to query
    /// * `form_time`: The start time of the query.
    /// * `to_time`: The time to end the search at.
    /// * `offset`: The offset of the first result to return.
    /// * `limit`: The number of results to return.
    ///
    /// Returns:
    ///
    /// A Result<SplTransfer, SolscanError>
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
    /// It gets the SOL transfers for a given account.
    ///
    /// Arguments:
    ///
    /// * `account`: The account address to query
    /// * `form_time`: The start time of the query.
    /// * `to_time`: The timestamp of the last block you want to include in the results.
    /// * `offset`: The offset of the first result to return.
    /// * `limit`: The number of results to return.
    ///
    /// Returns:
    ///
    /// A Result<SolTransferList, SolscanError>
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
    /// It fetches the account information of the given account.
    ///
    /// Arguments:
    ///
    /// * `account`: The account address to query
    ///
    /// Returns:
    ///
    /// A Result<AccountInfo, SolscanError>
    pub async fn get_account_account(&self, account: &str) -> Result<AccountInfo, SolscanError> {
        let url_endpoint: String = format!("/{}", account);
        self.solscan_fetch::<AccountInfo>(SolscanEndpoints::Account, url_endpoint.as_str()).await
    }
    //endregion

    //region Transaction
    /// It returns a list of token holders for a given token address.
    ///
    /// Arguments:
    ///
    /// * `account`: The address of the token contract
    /// * `offset`: The offset of the first result to return.
    /// * `limit`: The number of results to return.
    ///
    /// Returns:
    ///
    /// A Result<TokenHolders, SolscanError>
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
    /// It fetches the token meta data for a given token address.
    ///
    /// Arguments:
    ///
    /// * `account`: The address of the token contract
    ///
    /// Returns:
    ///
    /// A Result<TokenMeta, SolscanError>
    pub async fn get_token_meta(&self, account: &str) -> Result<TokenMeta, SolscanError> {
        let url_endpoint: String = format!("?tokenAddress={}", account);
        self.solscan_fetch::<TokenMeta>(SolscanEndpoints::TokenMeta, url_endpoint.as_str()).await
    }
    //endregion

    //region MarketToken
    /// It fetches the token market item for the given account.
    ///
    /// Arguments:
    ///
    /// * `account`: The address of the token contract
    ///
    /// Returns:
    ///
    /// A Result<TokenMarketItem, SolscanError>
    pub async fn get_market_token(&self, account: &str) -> Result<TokenMarketItem, SolscanError> {
        let url_endpoint: String = format!("/{}", account);
        self.solscan_fetch::<TokenMarketItem>(SolscanEndpoints::MarketToken, url_endpoint.as_str()).await
    }
    //endregion

    //region ChainInfo
    /// It gets the chain info from the Solana blockchain.
    ///
    /// Returns:
    ///
    /// A Result<ChainInfo, SolscanError>
    pub async fn get_chain_info(&self) -> Result<ChainInfo, SolscanError> {
        let url_endpoint: String = "/".to_string();
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