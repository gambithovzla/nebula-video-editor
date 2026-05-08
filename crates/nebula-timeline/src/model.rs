use nebula_types::{ClipId, SourceId, TimeSpan, TrackId};
use serde::{Deserialize, Serialize};

/// Kind of timeline track (video, audio, adjustment, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    Video,
    Audio,
    Adjustment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub kind: TrackKind,
    pub clips: Vec<ClipPlacement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClipPlacement {
    pub id: ClipId,
    pub source: SourceId,
    /// Start on the timeline.
    pub timeline_in: TimeSpan,
    /// Duration on the timeline (after speed / trim).
    pub timeline_duration: TimeSpan,
    /// In-point in source media.
    pub source_in: TimeSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: Vec<Track>,
}

impl Timeline {
    #[must_use]
    pub fn empty() -> Self {
        Self { tracks: Vec::new() }
    }
}
