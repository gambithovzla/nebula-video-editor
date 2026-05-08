# Nebula architecture

This document aligns the repo layout with the product plan: **professional editing**, **multicam-style multilayer timeline**, **GPU compositor**, **fast export**, **light on-device AI**, and **hybrid web/native** (desktop first).

## Layers

1. **UI** (`apps/desktop`): React + Vite + Tauri commands. Keeps layout, shortcuts, and panels; avoids heavy media work on the JS thread.
2. **Core** (`crates/*`): Timeline evaluation, decode scheduling, GPU composition, export, caches — intended to stay **platform-agnostic** Rust.
3. **Platform HAL** (future inside crates): `Decoder`, audio output, hardware encoders — cfg-gated per OS.

## Data flow (target)

- **Project** (`nebula-project`) is the canonical serialized document.
- **Timeline** (`nebula-timeline`) attaches to the project (today: parallel model; later: embedded blob or SQL).
- **Playback**: audio clock drives video frame scheduling (`nebula-audio` → decode → `nebula-render`).
- **Export**: flatten timeline → same graph as preview → encode/mux (`nebula-export`).

## Phases

- **Now**: types, project JSON, timeline stub, Tauri bridge, compositor stub.
- **Phase 0**: real decode (e.g. FFmpeg), first `wgpu` frame, A/V sync prototype.
- **Phase 1**: editing commands, undo log, export queue, SQLite library.
- **Phase 3**: `nebula-ffi` + UniFFI for Kotlin/Swift shells.

## Rust vs TypeScript boundary

Anything touching **bytes, real-time clocks, GPU, or codecs** stays in Rust. TypeScript issues **commands** and renders **panels** only.
