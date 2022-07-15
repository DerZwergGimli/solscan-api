//! # Solscan-API-Endpoints
//! This module represents the Solscan-API-Errors

/// Creating an enum for SolscanErrors.
#[derive(Debug)]
pub enum SolscanError {
    APIError,
    APIWrongStatusCode,
    APICConversionError,
    SerializeError,
}
