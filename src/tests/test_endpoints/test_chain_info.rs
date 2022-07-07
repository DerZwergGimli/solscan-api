#![allow(unused_variables)]

#[cfg(test)]
mod test_chain_info {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_chain_info::SAMPLE_CHAIN_INFO;
    use crate::tests::test_endpoints::sample_data::sample_market_token::SAMPLE_MARKET_TOKEN;
    use crate::tests::test_endpoints::sample_data::sample_token_meta::SAMPLE_TOKEN_META;

    #[tokio::test]
    async fn test_chain_info_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/chaininfo/");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_CHAIN_INFO);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_chain_info().await;
        assert_eq!(result.is_ok(), true)
    }
}