//! Audio-driven playback clock and (future) mix bus.
//!
//! Preview should treat the audio device timeline as authoritative to minimise A/V drift.

mod clock;
mod error;

pub use clock::{AudioClock, TransportState};
pub use error::AudioError;
