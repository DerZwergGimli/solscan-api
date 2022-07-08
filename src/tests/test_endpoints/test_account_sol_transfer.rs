#![allow(unused_variables)]

#[cfg(test)]
mod test_account_spl_transfer {
    use std::fs;

    use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::{json, Value};

    use crate::solscan::SolscanAPI;

    #[tokio::test]
    async fn test_account_spl_transfer_success() {
        let json_data_file = fs::read_to_string("./src/tests/test_endpoints/sample_data/sample_account_sol_transfer.json").expect("Unable to read file");
        let json_data: Value = serde_json::from_str(&json_data_file).expect("JSON does not have correct format.");
        let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);

        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/account/solTransfers")
                .query_param("account", "So11111111111111111111111111111111111111112");
            then.status(200)
                .header("content-type", "text/html")
                .json_body(json_data.clone());
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_account_sol_transfer("So11111111111111111111111111111111111111112", None, None, None, None).await.unwrap();

        assert_json_matches!(json!(&result), json_data, config)
    }
}