# Nebula — roadmap & session state

**Última actualización:** 2026-05-08  
**Mantén esta fecha al día** cuando cambies fase o completes hitos (humano o agente).

> **Uso para nuevas sesiones (IA / dev):** leer este archivo primero, luego [`ARCHITECTURE.md`](ARCHITECTURE.md) si toca diseño profundo.

---

## Dónde estamos (resumen)

| Fase        | Estado      | Notas breves |
|------------|-------------|----------------|
| **Phase 0**| En curso    | Preview primer fotograma vía FFmpeg CLI; sin playback continuo ni GPU preview aún. |
| **Phase 1**| No iniciada | Timeline persistente, edición, export, undo fuerte. |
| **Mobile** | No iniciada | `nebula-ffi` + UniFFI cuando el núcleo desktop sea estable. |

**Repo remoto:** [github.com/gambithovzla/nebula-video-editor](https://github.com/gambithovzla/nebula-video-editor)

---

## Hecho (checklist)

- [x] Monorepo Cargo + Tauri 2 + React (desktop).
- [x] Crates base: `types`, `project`, `timeline` (modelo), stubs decode/audio/render/export/cache/plugins/ml/ffi.
- [x] IPC `nebula_core_status` (smoke del workspace).
- [x] **Phase 0 — preview:** `ffprobe` + `ffmpeg` CLI en `nebula-decode::cli`; comando `video_preview_first_frame`; UI “Open video (first frame)” + `plugin-dialog`.
- [x] FFmpeg instalado en Windows de desarrollo (Gyan Essentials); documentado en README.
- [x] Iconos Tauri; `bundle.active: true`.

---

## En curso / siguiente (orden sugerido)

1. **Playback continuo** — cola de frames + reloj (audio-driven con **cpal** o equivalente).
2. **Preview en GPU** — `wgpu` + textura desde decode (sustituir o complementar PNG en UI).
3. **In-process decode** — enlazar FFmpeg/libs o Media Foundation; mantener trait `Decoder` para no acoplar UI.
4. **Proyecto ↔ timeline** — serializar `Timeline` en `Project` (dejar de usar solo stub en memoria).
5. **Export mínimo** — pipeline offline reutilizando grafo de preview.

*(Reordena o trocea según prioridad; al mover un ítem a “Hecho”, marca la casilla arriba y actualiza la fecha.)*

---

## Riesgos / dependencias externas

- **FFmpeg** debe estar en `PATH` para el preview actual (`ffmpeg`, `ffprobe`).
- **Windows:** MSVC Build Tools + WebView2 para Tauri.
- Sustituir CLI por libs implica definir estrategia de build/CI (vcpkg, artefactos, etc.).

---

## Anclas técnicas (dónde tocar qué)

| Área | Ubicación principal |
|------|---------------------|
| Preview / decode CLI | `crates/nebula-decode/src/cli.rs` |
| Comandos Tauri | `apps/desktop/src-tauri/src/lib.rs` |
| UI preview | `apps/desktop/src/App.tsx` |
| Permisos ventana | `apps/desktop/src-tauri/capabilities/default.json` |
| Modelo proyecto | `crates/nebula-project/` |
| Timeline | `crates/nebula-timeline/` |

---

## Cómo actualizar este documento

1. Cambia **Última actualización** a la fecha del cambio.
2. Mueve ítems entre **Hecho** y **En curso / siguiente**; marca `[x]` / `[ ]`.
3. Si cambia la fase global, edita la tabla **Dónde estamos**.
4. Añade una línea bajo **Hecho** o **siguiente** si aparece un nuevo hilo (ej. “tests E2E”, “CI”).
5. Haz commit con mensaje claro, ej. `docs(roadmap): phase 0 playback started`.

---

## Comandos rápidos

```bash
cargo check --workspace
cd apps/desktop && npm run build && npm run tauri dev
```
