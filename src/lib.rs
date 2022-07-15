//! # SOLSCAN-API
//! This lib represent a wrapper for the SolcanAPI
//!
//! # Solscan API
//!
//! [Solscan-Public-API](https://public-api.solscan.io/docs/#/)
//! `Default Limit: 150 requests/ 30 seconds, 100k requests / day`
//!
//!
//! # Example
//! Fetching last 10 Blocks form Solana-Blockchain via SolscanAPI
//! ```
//!async fn main() {
//!     let solscan_api = solscan_api::solscan::SolscanAPI::new();
//!     let result = solscan_api.get_block_last(Some(10)).await.unwrap();
//!
//!     println("{:?}", result)
//! }
//! ```

pub mod solscan;
mod r#const;
mod tests;
pub mod structs;
pub mod enums;

