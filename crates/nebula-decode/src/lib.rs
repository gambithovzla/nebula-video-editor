//! Media decode: frame pull model for the compositor and export pipeline.
//!
//! Concrete backends (FFmpeg, hardware decoders) are added per platform in later milestones.

mod cli;
mod error;
mod traits;

pub use cli::{first_frame_preview_png, probe_video_dimensions, VideoDimensions};
pub use error::DecodeError;
pub use traits::{DecodedFrame, Decoder, DecoderConfig};
