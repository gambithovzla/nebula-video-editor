# Nebula

Monorepo for a **GPU-first, multilayer** desktop video editor (CapCut-class editing, **no social**). This repository is an **early scaffold**: Rust workspace crates, a **Tauri 2 + React** shell, and a live `invoke` bridge to the core libraries.

## Prerequisites

- **Rust** (stable), **Cargo**, and the usual desktop toolchains for Tauri ([install](https://doc.rust-lang.org/cargo/getting-started/installation.html)).
- **Node.js** 20+ and **npm**.
- On Windows: **Visual Studio Build Tools 2022** with workload **“Desktop development with C++”** / **VC Tools** (provides `link.exe` for the MSVC target) and **WebView2** (usually preinstalled).

After installing Rust, open a **new** terminal so `%USERPROFILE%\.cargo\bin` is on your `PATH`, or add it manually.

Icons under `apps/desktop/src-tauri/icons/` were generated from `app-icon.png`. Replace that PNG and run `npm run tauri icon src-tauri/app-icon.png` from `apps/desktop` to refresh assets.

## Publish to GitHub

You need a one-time login: `gh auth login` (install [GitHub CLI](https://cli.github.com/) if needed).

Then from the repo root:

```powershell
.\scripts\github-init.ps1
```

That creates **`nebula-video-editor`** under your account, adds `origin`, and pushes **`main`**. If the name is taken, edit the script or run `gh repo create` manually.

Canonical repo: [github.com/gambithovzla/nebula-video-editor](https://github.com/gambithovzla/nebula-video-editor).

## Quick start

```bash
cd apps/desktop
npm install
npm run tauri dev
```

The window should open and call `nebula_core_status`, which exercises `nebula-project`, `nebula-timeline`, `nebula-audio`, and `nebula-render` from the workspace.

### Frontend / Rust only

```bash
cd apps/desktop
npm install
npm run dev          # Vite only (invoke will fail outside Tauri)
```

```bash
cargo check --workspace
```

## Layout

| Path | Role |
|------|------|
| `crates/nebula-types` | IDs, timebase, shared primitives |
| `crates/nebula-project` | Project document & schema version |
| `crates/nebula-timeline` | Tracks / clips model (stub) |
| `crates/nebula-decode` | Decoder trait (backends later) |
| `crates/nebula-audio` | Transport / clock (device later) |
| `crates/nebula-render` | Compositor stub → `wgpu` in Phase 0 |
| `crates/nebula-export` | Export job types |
| `crates/nebula-cache` | RAM budget / cache policy stub |
| `crates/nebula-plugins` | Plugin API version stub |
| `crates/nebula-ml` | Inference stub |
| `crates/nebula-ffi` | C ABI smoke hooks; UniFFI later |
| `apps/desktop` | Tauri + React UI |

Deep-dive design notes live in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

## Bundling

`tauri.conf.json` has **`bundle.active: false`** until you add icons under `apps/desktop/src-tauri/icons/`. Then run:

```bash
npm run tauri icon path/to/your.png
```

and set `bundle.active` to `true`.

## License

MIT OR Apache-2.0 (see crate metadata).
