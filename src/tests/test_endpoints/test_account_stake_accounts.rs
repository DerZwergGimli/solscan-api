#![allow(unused_variables)]

#[cfg(test)]
mod test_account_stake_accounts {
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::json;
    use test_env_log::test;

    use crate::enums::solscan_endpoints::SolscanEndpoints;
    use crate::solscan::SolscanAPI;
    use crate::structs::transaction::Transaction;
    use crate::tests::test_endpoints::sample_data::sample_account_stake_account::SAMPLE_ACCOUNT_STAKE_ACCOUNT;

    #[tokio::test]
    async fn test_account_stake_accounts_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/stakeAccounts")
                .query_param("account", "55bmPBVG1xNHSmQ9iqzvm78Gvr8rE9wiG48MLQgJ1ywA");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_ACCOUNT_STAKE_ACCOUNT);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_stake_accounts("55bmPBVG1xNHSmQ9iqzvm78Gvr8rE9wiG48MLQgJ1ywA").await.unwrap();
        //assert!(result.unwrap())
    }
}