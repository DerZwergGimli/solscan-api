#![allow(unused_variables)]

#[cfg(test)]
mod test_token_holders {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_token_holders::SAMPLE_TOKEN_HOLDERS;

    #[tokio::test]
    async fn test_token_holders_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/token/holders")
                .query_param("tokenAddress", "So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_TOKEN_HOLDERS);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_token_holders("So11111111111111111111111111111111111111112", None, None).await;
        assert_eq!(result.is_ok(), true)
    }
}