use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("decoder not ready")]
    NotReady,
    #[error("end of stream")]
    EndOfStream,
    #[error("decode failed: {0}")]
    Backend(String),
}
