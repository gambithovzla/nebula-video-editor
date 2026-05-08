use crate::RenderError;

/// Owns GPU device/queue and (eventually) the frame graph.
#[derive(Debug, Default)]
pub struct RenderEngine;

impl RenderEngine {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Placeholder until `wgpu` surfaces and pipelines are wired (Phase 0).
    ///
    /// # Errors
    /// Currently returns [`RenderError::NotReady`].
    pub fn health_check(&self) -> Result<(), RenderError> {
        Err(RenderError::NotReady)
    }
}
