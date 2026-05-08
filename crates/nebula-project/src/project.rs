use nebula_types::{FrameRate, ProjectId, TimeSpan};
use serde::{Deserialize, Serialize};

use crate::ProjectError;

/// Increment when breaking the on-disk project format.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);

/// Wrapper for explicit schema evolution in serde.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub id: ProjectId,
    pub name: String,
    pub frame_rate: FrameRate,
    pub duration: TimeSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub schema: SchemaVersion,
    pub metadata: ProjectMetadata,
    /// Opaque payload reserved for timeline graph (populated by `nebula-timeline` in later phases).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_blob: Option<serde_json::Value>,
}

impl Project {
    /// Builds a minimal empty project with the current schema version.
    #[must_use]
    pub fn new(name: impl Into<String>, frame_rate: FrameRate) -> Self {
        Self {
            schema: CURRENT_SCHEMA_VERSION,
            metadata: ProjectMetadata {
                id: ProjectId::new(),
                name: name.into(),
                frame_rate,
                duration: TimeSpan::from_seconds_unchecked(0.0),
            },
            timeline_blob: None,
        }
    }

    /// Serializes to pretty JSON for human-editable project files.
    ///
    /// # Errors
    /// Returns [`ProjectError::Json`] on serialization failure.
    pub fn to_json_pretty(&self) -> Result<String, ProjectError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Deserializes from JSON, rejecting unknown future schema versions.
    ///
    /// # Errors
    /// Returns [`ProjectError::UnsupportedSchema`] if `schema` is greater than [`CURRENT_SCHEMA_VERSION`].
    pub fn from_json_slice(data: &[u8]) -> Result<Self, ProjectError> {
        let project: Project = serde_json::from_slice(data)?;
        if project.schema.0 > CURRENT_SCHEMA_VERSION.0 {
            return Err(ProjectError::UnsupportedSchema(project.schema.0));
        }
        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_json() {
        let fr = FrameRate::from_rational(24, 1).unwrap();
        let p = Project::new("Test", fr);
        let s = p.to_json_pretty().unwrap();
        let again = Project::from_json_slice(s.as_bytes()).unwrap();
        assert_eq!(p.metadata.name, again.metadata.name);
        assert_eq!(p.schema, again.schema);
    }
}
