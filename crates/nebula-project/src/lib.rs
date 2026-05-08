//! On-disk project representation and versioning.
//!
//! The UI and FFI layers should treat [`Project`] as the canonical document model.

mod error;
mod project;

pub use error::ProjectError;
pub use project::{Project, ProjectMetadata, SchemaVersion, CURRENT_SCHEMA_VERSION};
