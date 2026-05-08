use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("export cancelled")]
    Cancelled,
    #[error("encoder failed: {0}")]
    Encoder(String),
}
