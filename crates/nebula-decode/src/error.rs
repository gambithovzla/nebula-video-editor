use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("decoder not ready")]
    NotReady,
    #[error("end of stream")]
    EndOfStream,
    #[error("decode failed: {0}")]
    Backend(String),
    #[error("I/O running ffmpeg/ffprobe: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid probe or frame data")]
    InvalidData,
    #[error("unexpected frame buffer size (got {got}, expected {expected})")]
    BadFrameSize { got: usize, expected: usize },
    #[error("could not parse ffprobe JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("image encode failed: {0}")]
    Image(#[from] image::ImageError),
    #[error("{tool} failed ({code:?}): {msg}")]
    ProcessFailed {
        tool: &'static str,
        code: Option<i32>,
        msg: String,
    },
}

impl DecodeError {
    pub(crate) fn process(tool: &'static str, stderr: &[u8], code: Option<i32>) -> Self {
        let msg = String::from_utf8_lossy(stderr).trim().to_string();
        let msg = if msg.is_empty() {
            "no stderr".into()
        } else {
            msg
        };
        Self::ProcessFailed { tool, code, msg }
    }
}
