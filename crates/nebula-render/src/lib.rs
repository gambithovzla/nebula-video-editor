//! GPU composition: texture pool, shaders, and presentation.
//!
//! Enable the `gpu` feature to pull in `wgpu`; the default build stays lightweight for CI/agents.

mod error;
mod engine;

pub use engine::RenderEngine;
pub use error::RenderError;
