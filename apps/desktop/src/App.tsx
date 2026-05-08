import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useState } from "react";

type CoreStatus = {
  schemaVersion: number;
  projectName: string;
  timelineTrackCount: number;
  audioPositionSecs: number;
  renderReady: boolean;
};

type VideoDimensions = {
  width: number;
  height: number;
};

type VideoPreview = {
  source: VideoDimensions;
  preview: VideoDimensions;
  pngBase64: string;
};

export default function App() {
  const [status, setStatus] = useState<CoreStatus | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [preview, setPreview] = useState<VideoPreview | null>(null);
  const [previewLoading, setPreviewLoading] = useState(false);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const payload = await invoke<CoreStatus>("nebula_core_status");
      setStatus(payload);
    } catch (e) {
      setStatus(null);
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const openVideoPreview = useCallback(async () => {
    setError(null);
    setPreview(null);
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Video",
          extensions: ["mp4", "mkv", "mov", "webm", "avi", "m4v", "mpeg", "mpg"],
        },
      ],
    });
    const path =
      selected === null
        ? null
        : Array.isArray(selected)
          ? selected[0] ?? null
          : selected;
    if (!path) {
      return;
    }
    setPreviewLoading(true);
    try {
      const payload = await invoke<VideoPreview>("video_preview_first_frame", { path });
      setPreview(payload);
    } catch (e) {
      setPreview(null);
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setPreviewLoading(false);
    }
  }, []);

  return (
    <div className="app">
      <header>
        <h1>Nebula</h1>
        <p>GPU-first multilayer editor — Phase 0: first-frame preview (FFmpeg)</p>
      </header>
      <main>
        <div className="actions">
          <button type="button" onClick={() => void refresh()} disabled={loading}>
            {loading ? "Loading…" : "Refresh core status"}
          </button>
          <button type="button" onClick={() => void openVideoPreview()} disabled={previewLoading}>
            {previewLoading ? "Decoding…" : "Open video (first frame)"}
          </button>
        </div>

        {error ? (
          <p className="error" role="alert">
            {error}
          </p>
        ) : null}

        {preview ? (
          <section className="panel preview-panel" aria-label="Video preview">
            <h2>First frame</h2>
            <p className="meta">
              Source {preview.source.width}×{preview.source.height} → preview {preview.preview.width}×
              {preview.preview.height}
            </p>
            <img
              className="preview-img"
              src={`data:image/png;base64,${preview.pngBase64}`}
              alt="First decoded frame"
            />
          </section>
        ) : null}

        {status ? (
          <>
            <section className="panel" aria-live="polite">
              <h2>Rust workspace</h2>
              <dl>
                <dt>Schema version</dt>
                <dd>{status.schemaVersion}</dd>
                <dt>Project</dt>
                <dd>{status.projectName}</dd>
                <dt>Timeline tracks (stub)</dt>
                <dd>{status.timelineTrackCount}</dd>
                <dt>Audio clock (stub)</dt>
                <dd>{status.audioPositionSecs.toFixed(3)} s</dd>
                <dt>Render engine</dt>
                <dd>{status.renderReady ? "ready" : "not initialised (expected)"}</dd>
              </dl>
            </section>
            <section className="panel">
              <h2>Raw payload</h2>
              <pre>{JSON.stringify(status, null, 2)}</pre>
            </section>
          </>
        ) : null}
      </main>
      <footer>
        <code>video_preview_first_frame</code> uses FFmpeg/ffprobe on your PATH (max preview width 1280).
      </footer>
    </div>
  );
}
