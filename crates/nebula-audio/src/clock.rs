/// High-level transport state for the preview engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransportState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

/// Monotonic playback position in seconds (UI-facing; device timestamps come later).
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioClock {
    position_secs: f64,
    state: TransportState,
}

impl AudioClock {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            position_secs: 0.0,
            state: TransportState::Stopped,
        }
    }

    #[must_use]
    pub fn position_secs(&self) -> f64 {
        self.position_secs
    }

    #[must_use]
    pub fn state(&self) -> TransportState {
        self.state
    }

    pub fn seek(&mut self, secs: f64) {
        self.position_secs = secs.max(0.0);
    }

    pub fn play(&mut self) {
        self.state = TransportState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = TransportState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = TransportState::Stopped;
    }

    /// Advances the clock by `delta_secs` when playing.
    pub fn tick(&mut self, delta_secs: f64) {
        if self.state == TransportState::Playing && delta_secs > 0.0 {
            self.position_secs += delta_secs;
        }
    }
}
