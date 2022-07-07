use log::error;
use reqwest::{Client, Error, StatusCode};
use serde::de::DeserializeOwned;

use crate::enums::solscan_endpoints::SolscanEndpoints;
use crate::enums::solscan_errors::SolscanError;
use crate::r#const::SOLSCANBASEURL;
use crate::structs::block::Block;
use crate::structs::token::Token;

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

    pub async fn solscan_fetch<T: DeserializeOwned>(&self, endpoint: SolscanEndpoints, account: Option<String>, block: Option<i64>, offset: Option<i64>, limit: Option<i64>) -> Result<T, SolscanError> {
        match {
            self.
                fetch(endpoint.value().to_owned() +
                    account.unwrap_or("".to_string()).as_str()
                ).await
        } {
            Ok(api_data) => {
                match serde_json::from_str::<T>(api_data.as_str()) {
                    Ok(api_data) => {
                        Ok(api_data)
                    }
                    Err(e) => {
                        error!("{:?}",e);
                        Err(SolscanError::SerializeError)
                    }
                }
            }
            Err(e) => {
                error!("{:?}", e);
                Err(SolscanError::APIError)
            }
        }
    }
    //endregion

    //region public functions
    pub async fn ping_status(&self, endpoint: Option<String>) -> Result<StatusCode, Error> {
        Ok(self.client.get(self.base_url.to_string() + endpoint.unwrap_or_default().as_str()).header("User-Agent", "Mozilla/5.0").send().await?.status())
    }


    //region block
    pub async fn get_block_last(&self) -> Result<Vec<Block>, SolscanError> {
        self.solscan_fetch::<Vec<Block>>(SolscanEndpoints::BlockLast, None, None, None, None).await
    }
    pub async fn get_block_transactions(&self, block: i64, offset: i64, limit: i64) -> Result<Vec<Block>, SolscanError> {
        self.solscan_fetch::<Vec<Block>>(SolscanEndpoints::BlockTransactions, None, Some(block), Some(offset), Some(limit)).await
    }
    //endregion

    pub async fn get_account_tokens(&self, account: &str) -> Result<Vec<Token>, SolscanError> {
        self.solscan_fetch::<Vec<Token>>(SolscanEndpoints::AccountTokens, Some(account.to_string()), None, None, None).await
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