//! Foreign-function interface for iOS/Android shells.
//!
//! When mobile work starts, add `uniffi` and a `.udl` or proc-macro scaffolding here.

use nebula_project::Project;
use nebula_types::FrameRate;

/// Minimal C-ABI style hook for sanity checks from native debug builds.
/// Prefer UniFFI-generated bindings for production APIs.
#[no_mangle]
pub extern "C" fn nebula_ffi_schema_version() -> u32 {
    nebula_project::CURRENT_SCHEMA_VERSION.0
}

/// Returns 1 if a trivial project serializes; 0 on failure (for JNI smoke tests).
#[no_mangle]
pub extern "C" fn nebula_ffi_self_test() -> i32 {
    let fr = match FrameRate::from_rational(24, 1) {
        Ok(f) => f,
        Err(_) => return 0,
    };
    let p = Project::new("ffi", fr);
    match p.to_json_pretty() {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
