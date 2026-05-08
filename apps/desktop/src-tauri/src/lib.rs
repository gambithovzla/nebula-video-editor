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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![nebula_core_status])
        .run(tauri::generate_context!())
        .expect("error while starting Nebula");
}
