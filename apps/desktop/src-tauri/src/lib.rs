use std::path::PathBuf;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use nebula_audio::AudioClock;
use nebula_project::Project;
use nebula_render::RenderEngine;
use nebula_timeline::{Timeline, Track, TrackKind};
use nebula_types::{FrameRate, TrackId};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreStatusPayload {
    pub schema_version: u32,
    pub project_name: String,
    pub timeline_track_count: usize,
    pub audio_position_secs: f64,
    pub render_ready: bool,
}

#[tauri::command]
fn nebula_core_status() -> CoreStatusPayload {
    let fr = FrameRate::from_rational(24, 1).expect("valid frame rate");
    let project = Project::new("Untitled", fr);
    let track = Track {
        id: TrackId::new(),
        kind: TrackKind::Video,
        clips: vec![],
    };
    let timeline = Timeline {
        tracks: vec![track],
    };
    let mut clock = AudioClock::new();
    clock.seek(0.0);
    let engine = RenderEngine::new();
    let render_ready = engine.health_check().is_ok();

    CoreStatusPayload {
        schema_version: nebula_project::CURRENT_SCHEMA_VERSION.0,
        project_name: project.metadata.name.clone(),
        timeline_track_count: timeline.tracks.len(),
        audio_position_secs: clock.position_secs(),
        render_ready,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPreviewPayload {
    pub source: nebula_decode::VideoDimensions,
    pub preview: nebula_decode::VideoDimensions,
    pub png_base64: String,
}

/// First-frame preview (PNG) using FFmpeg CLI on `PATH`. Max width 1280 px.
#[tauri::command]
fn video_preview_first_frame(path: String) -> Result<VideoPreviewPayload, String> {
    let path = PathBuf::from(path.trim());
    if !path.is_file() {
        return Err(format!("not a file: {}", path.display()));
    }
    let (source, preview, png) = nebula_decode::first_frame_preview_png(&path, 1280)
        .map_err(|e| e.to_string())?;
    Ok(VideoPreviewPayload {
        source,
        preview,
        png_base64: STANDARD.encode(png),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![nebula_core_status, video_preview_first_frame])
        .run(tauri::generate_context!())
        .expect("error while starting Nebula");
}
