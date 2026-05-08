use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("render engine not initialised")]
    NotReady,
    #[error("GPU error: {0}")]
    Gpu(String),
}
