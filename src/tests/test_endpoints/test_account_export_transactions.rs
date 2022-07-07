#![allow(unused_variables)]

#[cfg(test)]
mod test_account_export_transactions {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_account_stake_account::SAMPLE_ACCOUNT_STAKE_ACCOUNT;

    #[tokio::test]
    async fn test_account_export_transactions_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/stakeAccounts")
                .query_param("account", "So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_ACCOUNT_STAKE_ACCOUNT);
        });

        //let solscan_api = SolscanAPI::new_with_url(server.url(""));
        //let result = solscan_api.get_account_stake_accounts("So11111111111111111111111111111111111111112").await.unwrap();
        assert_eq!(true, false)
    }
}