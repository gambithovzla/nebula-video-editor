use thiserror::Error;

/// Errors surfaced by low-level type validation (e.g. invalid frame rate).
#[derive(Debug, Error)]
pub enum TypesError {
    #[error("frame rate must be positive, got {0}")]
    InvalidFrameRate(f64),
    #[error("time span duration must be non-negative")]
    NegativeDuration,
}
