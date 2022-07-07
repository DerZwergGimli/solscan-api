#![allow(unused_variables)]

#[cfg(test)]
mod test_api {
    use crate::solscan::SolscanAPI;

    #[tokio::test]
    async fn test_api_fetch_block_last() {
        let solscan_api = SolscanAPI::new();
        let result = solscan_api.get_block_last().await.unwrap();
    }
}