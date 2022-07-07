#![allow(unused_variables)]

#[cfg(test)]
mod test_block_last {
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::json;
    use test_env_log::test;

    use crate::enums::solscan_endpoints::SolscanEndpoints;
    use crate::solscan::SolscanAPI;
    use crate::structs::transaction::Transaction;
    use crate::tests::test_endpoints::sample_data::sample_block_last::SAMPLE_BLOCK_LAST;

    #[tokio::test]
    async fn test_block_last_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/block/last");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_BLOCK_LAST);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_block_last().await.unwrap();
        assert_eq!(result.len(), 10)
    }
}