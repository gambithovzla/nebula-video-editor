//! Multilayer timeline structures and evaluation hooks.
//!
//! Phase 1 will add interval indices and a GPU composition plan; for now this crate
//! defines stable IDs and track/clip layout.

mod error;
mod model;

pub use error::TimelineError;
pub use model::{ClipPlacement, Timeline, Track, TrackKind};
