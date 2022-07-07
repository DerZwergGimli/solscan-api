#![allow(unused_variables)]

#[cfg(test)]
mod test_transaction_signature {
    use httpmock::MockServer;
    use httpmock::prelude::*;
    use serde_json::json;

    use crate::enums::solscan_endpoints::SolscanEndpoints;
    use crate::solscan::SolscanAPI;
    use crate::structs::transaction::Transaction;
    use crate::tests::test_endpoints::sample_data::sample_transaction_signature::SAMPLE_TRANSACTION_SIGNATURE;

    #[tokio::test]
    async fn test_transaction_signature_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/transaction/T4ipYTjKUqHQpfuA8ZM5E4iJag9kX9nGhjbY974oq2ucyYRL6eWhqTjtmk3cqfqTSu8Qdce33vzKQd7bWEX3H21");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_TRANSACTION_SIGNATURE)
            ;
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_transaction("T4ipYTjKUqHQpfuA8ZM5E4iJag9kX9nGhjbY974oq2ucyYRL6eWhqTjtmk3cqfqTSu8Qdce33vzKQd7bWEX3H21".to_string()).await.unwrap();
    }
}