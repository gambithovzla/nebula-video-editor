import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";

type CoreStatus = {
  schemaVersion: number;
  projectName: string;
  timelineTrackCount: number;
  audioPositionSecs: number;
  renderReady: boolean;
};

export default function App() {
  const [status, setStatus] = useState<CoreStatus | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

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

  return (
    <div className="app">
      <header>
        <h1>Nebula</h1>
        <p>GPU-first multilayer editor — workspace scaffold</p>
      </header>
      <main>
        <button type="button" onClick={() => void refresh()} disabled={loading}>
          {loading ? "Loading…" : "Refresh core status"}
        </button>

        {error ? (
          <p className="error" role="alert">
            {error}
          </p>
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
        Invoke <code>nebula_core_status</code> — wires UI to workspace crates via Tauri.
      </footer>
    </div>
  );
}
