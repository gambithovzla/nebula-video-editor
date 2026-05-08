use nebula_types::{SourceId, TimeSpan};

use crate::DecodeError;

/// Immutable description passed when opening a decoder.
#[derive(Debug, Clone)]
pub struct DecoderConfig {
    pub source: SourceId,
    pub path: std::path::PathBuf,
}

/// One video frame in an opaque, backend-specific representation until GPU upload is defined.
#[derive(Debug)]
pub struct DecodedFrame {
    pub timestamp: TimeSpan,
    /// Byte length or handle id depending on backend; placeholder for Phase 0.
    pub payload_len: usize,
}

/// Pull-based decoder for a single asset.
pub trait Decoder: Send {
    /// Advances decode and returns the next frame in presentation order.
    fn next_frame(&mut self) -> Result<DecodedFrame, DecodeError>;
}
