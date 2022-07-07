#![allow(unused_variables)]

#[cfg(test)]
mod test_block_last {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_block_block::SAMPLE_BLOCK_BLOCK;

    #[tokio::test]
    async fn test_block_last_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/block/140604012");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_BLOCK_BLOCK);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_block_block(140604012).await.unwrap();
        //assert!(result.unwrap())
    }
}