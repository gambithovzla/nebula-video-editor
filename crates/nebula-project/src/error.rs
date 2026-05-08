use thiserror::Error;

use nebula_types::TypesError;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("unsupported project schema version: {0}")]
    UnsupportedSchema(u32),
    #[error("JSON (de)serialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("type validation failed: {0}")]
    Types(#[from] TypesError),
}
