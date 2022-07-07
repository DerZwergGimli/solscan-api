#![allow(unused_variables)]

#[cfg(test)]
mod test_market_token {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_market_token::SAMPLE_MARKET_TOKEN;
    use crate::tests::test_endpoints::sample_data::sample_token_meta::SAMPLE_TOKEN_META;

    #[tokio::test]
    async fn test_market_token_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/market/token/So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_MARKET_TOKEN);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_market_token("So11111111111111111111111111111111111111112").await;
        assert_eq!(result.is_ok(), true)
    }
}