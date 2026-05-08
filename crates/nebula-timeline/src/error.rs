use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("clip placement overlaps existing clip on track {0:?}")]
    Overlap(nebula_types::TrackId),
}
