#[cfg(test)]
mod test_api {
    use crate::solscan::SolscanAPI;

    #[tokio::test]
    async fn test_api_ping_404() {
        let solscan_api = SolscanAPI::new();
        let result = solscan_api.ping(None).await.unwrap();
        assert_eq!(result, 404)
    }

    #[tokio::test]
    async fn test_api_ping_200() {
        let solscan_api = SolscanAPI::new();
        let result = solscan_api.ping(Some("/block/last".to_string())).await.unwrap();
        assert_eq!(result, 200)
    }
}