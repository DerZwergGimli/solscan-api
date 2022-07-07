#![allow(unused_variables)]

#[cfg(test)]
mod test_account_transactions {
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::json;
    use test_env_log::test;

    use crate::enums::solscan_endpoints::SolscanEndpoints;
    use crate::solscan::SolscanAPI;
    use crate::structs::transaction::Transaction;
    use crate::tests::test_endpoints::sample_data::sample_account_transactions::SAMPLE_ACCOUNT_TRANSACTION;

    #[tokio::test]
    async fn test_account_transactions_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/transactions")
                .query_param("account", "So11111111111111111111111111111111111111112")
                .query_param("limit", "10");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_ACCOUNT_TRANSACTION);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_transactions("So11111111111111111111111111111111111111112", None, Some(10)).await.unwrap();
        assert_eq!(result.len(), 10)
    }
}