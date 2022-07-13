#![allow(unused_variables)]

#[cfg(test)]
mod test_transaction_signature {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;
    use crate::tests::test_endpoints::sample_data::sample_transaction_signature::SAMPLE_TRANSACTION_SIGNATURE;

    #[tokio::test]
    async fn test_transaction_signature_success() {
        let server = MockServer::start();
        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/transaction/2DeFa7rvwudbY7f4XkUrJ9RfUBTjT6CmFHkgqnNpE4cvaWRpRJEkx4myTpmLpQsaKvDmtMKQf6KNsdrgGNxPCnVY");
            then.status(200)
                .header("content-type", "text/html")
                .body(SAMPLE_TRANSACTION_SIGNATURE)
            ;
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.get_transaction("2DeFa7rvwudbY7f4XkUrJ9RfUBTjT6CmFHkgqnNpE4cvaWRpRJEkx4myTpmLpQsaKvDmtMKQf6KNsdrgGNxPCnVY").await.unwrap();
    }
}