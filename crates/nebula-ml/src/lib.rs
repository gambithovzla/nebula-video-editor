//! Optional ML subgraphs (segmentation, light enhancement, etc.).

mod error;

pub use error::MlError;

/// Placeholder handle until ONNX sessions are wired.
#[derive(Debug, Default)]
pub struct InferenceSession;

impl InferenceSession {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}
