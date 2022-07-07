#![allow(unused_variables)]

#[cfg(test)]
mod test_mock_api {
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::json;

    use crate::enums::solscan_endpoints::SolscanEndpoints;
    use crate::solscan::SolscanAPI;
    use crate::structs::transaction::Transaction;

    /*    fn build_mock_server_block() -> MockServer {
                                let server = MockServer::start();

                                let mock_block = server.mock(|when, then| {
                                    when.method(GET)
                                        .path("/block/last");
                                    then.status(200)
                                        .header("content-type", "text/html")
                                        .json_body(json!([
                                                                  {
                                                                    "currentSlot": 140470240,
                                                                    "result": {
                                                                      "blockHeight": 126891281,
                                                                      "blockTime": 1657124585,
                                                                      "blockhash": "HijFFgfAfeawaKUZ3UdBSw5Qyi5EfhwszH6S9CkMa6nC",
                                                                      "parentSlot": 140470239,
                                                                      "previousBlockhash": "FjxsV8xmR5AB4AU8sBFJDvm8Cexgx8BeXaiarNvkKFH",
                                                                      "feeRewards": 5883517,
                                                                      "validator": "2mMGsb5uy1Q4Dvezr8HK2E8SJoChcb2X7b61tJPaVHHd",
                                                                      "transactionCount": 2322
                                                                    }
                                                                  }]));
                                });
                                server
                            }*/

    fn build_mock_server_account_tokens() -> MockServer {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/tokens")
                .query_param("account", "So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .json_body(json!([
                                      {
                                        "tokenAddress": "Fh6Vvw44wb6n5honzAfvrae6iq7FSRiKzMB6runL8csb",
                                        "tokenAmount": {
                                          "amount": "1",
                                          "decimals": 0,
                                          "uiAmount": 1,
                                          "uiAmountString": "1"
                                        },
                                        "tokenAccount": "Cexn2xqP9ruDzp2Pmrg4ahFBMKHyULdjwnCeVBSGQkFh",
                                        "tokenName": "",
                                        "tokenIcon": "",
                                        "rentEpoch": 324,
                                        "lamports": 2039280
                                      }]));
        });
        server
    }

    fn build_mock_server_error() -> MockServer {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/block/last");
            then.status(400)
                .header("content-type", "text/html");
        });
        server
    }

    //region ErrorCodesTest

    #[tokio::test]
    async fn test_mock_api_200() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(200);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 200)
    }

    #[tokio::test]
    async fn test_mock_api_400() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(400);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 400)
    }

    #[tokio::test]
    async fn test_mock_api_429() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(429);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 429)
    }

    #[tokio::test]
    async fn test_mock_api_500() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(500);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 500)
    }

    //endregion

    /*    #[tokio::test]
        async fn test_mock_api_fetch_block_last_success() {
            let server = build_mock_server_block();

            let solscan_api = SolscanAPI::new_with_url(server.url(""));
            let result = solscan_api.get_block_last().await.unwrap();
            assert_eq!(result.len(), 1)
        }*/

    #[tokio::test]
    async fn test_mock_api_fetch_account_tokens_last_success() {
        let server = build_mock_server_account_tokens();

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_tokens("So11111111111111111111111111111111111111112").await.unwrap();
        assert_eq!(result.len(), 1)
    }

    /*#[tokio::test]
    async fn test_mock_api_fetch_account_token_parse_error() {
        let server = build_mock_server_block();

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.solscan_fetch::<Vec<Transaction>>(SolscanEndpoints::BlockLast, None, None, None, None).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_mock_api_fetch_account_token_parse_error2() {
        let server = build_mock_server_error();

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.solscan_fetch::<Vec<Transaction>>(SolscanEndpoints::BlockLast, None, None, None, None).await;
        assert!(result.is_err())
    }*/
}