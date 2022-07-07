#![allow(unused_variables)]

#[cfg(test)]
mod test_account_export_transactions {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_account_account::SAMPLE_ACCOUNT_ACCOUNT;
    use crate::tests::test_endpoints::sample_data::sample_account_stake_account::SAMPLE_ACCOUNT_STAKE_ACCOUNT;

    #[tokio::test]
    async fn test_account_export_transactions_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_ACCOUNT_ACCOUNT);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_account("So11111111111111111111111111111111111111112").await;
        assert_eq!(result.is_ok(), true)
    }
}