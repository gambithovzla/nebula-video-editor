use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("audio backend error: {0}")]
    Backend(String),
}
