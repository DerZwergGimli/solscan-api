#![allow(unused_variables)]

#[cfg(test)]
mod test_block_transaction {
    use std::fs;

    use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::{json, Value};

    use crate::solscan::SolscanAPI;

    #[tokio::test]
    async fn test_block_transaction_success() {
        let json_data_file = fs::read_to_string("./src/tests/test_endpoints/sample_data/sample_block_transactions.json").expect("Unable to read file");
        let json_data: Value = serde_json::from_str(&json_data_file).expect("JSON does not have correct format.");
        let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);

        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/block/transactions")
                .query_param("block", "141591240")
                .query_param("offset", "0")
                .query_param("limit", "50");

            then.status(200)
                .header("content-type", "text/html")
                .json_body(json_data.clone());
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_block_transactions(141591240, Some(0), Some(50)).await.unwrap();

        assert_json_matches!(json!(&result), json_data, config)
    }
}