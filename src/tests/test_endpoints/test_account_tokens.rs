#![allow(unused_variables)]

#[cfg(test)]
mod test_account_tokens {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_account_tokens::SAMPLE_ACCOUNT_TOKENS;

    #[tokio::test]
    async fn test_account_tokens_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/tokens")
                .query_param("account", "So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_ACCOUNT_TOKENS);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_tokens("So11111111111111111111111111111111111111112").await.unwrap();
        //assert!(result.unwrap())
    }
}