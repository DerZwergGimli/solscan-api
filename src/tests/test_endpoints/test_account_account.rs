#![allow(unused_variables)]

#[cfg(test)]
mod test_account_account {
    use assert_json_diff::{assert_json_eq, assert_json_include};
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::json;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_account_account::SAMPLE_ACCOUNT_ACCOUNT;

    #[tokio::test]
    async fn test_account_account_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_ACCOUNT_ACCOUNT);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_account("So11111111111111111111111111111111111111112").await.unwrap();

        assert_json_eq!(json!(serde_json::to_string(&result).unwrap()), json!(SAMPLE_ACCOUNT_ACCOUNT))
        //assert_eq!(serde_json::to_string(&result).unwrap(), serde_json::to_string(SAMPLE_ACCOUNT_ACCOUNT).unwrap())
//        assert_eq!(format!("{:?}", result), format!("{:?}", SAMPLE_ACCOUNT_ACCOUNT))
    }
}