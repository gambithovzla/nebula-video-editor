use nebula_project::Project;
use serde::{Deserialize, Serialize};

/// User-facing export quality preset (maps to codec params per platform later).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPreset {
    Draft,
    Standard,
    Pro,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExportProgress {
    pub done_frames: u64,
    pub total_frames: u64,
}

/// Owns export state; execution will spawn workers in Phase 1.
#[derive(Debug, Clone)]
pub struct ExportJob {
    pub project: Project,
    pub preset: ExportPreset,
}

impl ExportJob {
    #[must_use]
    pub fn new(project: Project, preset: ExportPreset) -> Self {
        Self { project, preset }
    }

    /// Placeholder progress until the encoder loop exists.
    #[must_use]
    pub fn stub_progress(&self) -> ExportProgress {
        ExportProgress {
            done_frames: 0,
            total_frames: 1,
        }
    }
}
