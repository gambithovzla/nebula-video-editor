//! Media decode: frame pull model for the compositor and export pipeline.
//!
//! Concrete backends (FFmpeg, hardware decoders) are added per platform in later milestones.

mod error;
mod traits;

pub use error::DecodeError;
pub use traits::{DecodedFrame, Decoder, DecoderConfig};
