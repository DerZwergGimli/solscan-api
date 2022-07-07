#![allow(unused_variables)]

#[cfg(test)]
mod test_error_codes {
    use httpmock::MockServer;
    use httpmock::prelude::*;

    use crate::solscan::SolscanAPI;

    #[tokio::test]
    async fn test_error_code_200_success() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(200);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 200)
    }

    #[tokio::test]
    async fn test_error_code_400_success() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(400);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 400)
    }

    #[tokio::test]
    async fn test_error_code_429_success() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(429);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 429)
    }

    #[tokio::test]
    async fn test_error_code_500_success() {
        let server = MockServer::start();

        let mock_block = server.mock(|when, then| {
            when.method(GET)
                .path("/test");
            then.status(500);
        });

        let solscan_api = SolscanAPI::new_with_url(server.url(""));
        let result = solscan_api.ping_status(Some("/test".to_string())).await.unwrap();
        assert_eq!(result, 500)
    }
}