#[derive(Debug)]
pub enum SolscanError {
    APIError,
    APIWrongStatusCode,
    APICConversionError,
    SerializeError,
}
