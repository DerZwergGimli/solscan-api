//! # Solscan-API-Endpoints
//! This module represents the Solscan-API-Errors

#[derive(Debug)]
pub enum SolscanError {
    APIError,
    APIWrongStatusCode,
    APICConversionError,
    SerializeError,
}
