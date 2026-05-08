# Agent / contributor notes

- **Scope**: Prefer changes inside the crate that owns the concern; avoid cross-crate API churn unless necessary.
- **Build**: After editing Rust, run `cargo check --workspace`. After editing the UI, run `npm run build` in `apps/desktop`.
- **Tauri**: New IPC commands belong in `apps/desktop/src-tauri/src/lib.rs` with typed payloads (`serde`).
- **MSRV**: Workspace `rust-version` is authoritative; avoid features above it without updating the toolchain file.
- **Docs**: Architecture intent lives in `docs/ARCHITECTURE.md`; keep README short and actionable.
