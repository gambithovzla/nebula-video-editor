use thiserror::Error;

#[derive(Debug, Error)]
pub enum MlError {
    #[error("inference failed: {0}")]
    Inference(String),
}
