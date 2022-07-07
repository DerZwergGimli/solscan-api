#![allow(unused_variables)]

#[cfg(test)]
mod test_transaction_last {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_block_transactions::SAMPLE_BLOCK_TRANSACTIONS;

    #[tokio::test]
    async fn test_transaction_last_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/transaction/last")
                .query_param("limit", "10");

            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_BLOCK_TRANSACTIONS)
            ;
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_transaction_last(Some(10)).await.unwrap();
        assert_eq!(result.len(), 10)
    }
}