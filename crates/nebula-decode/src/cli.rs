//! Decode first frames by shelling out to `ffmpeg` / `ffprobe` (must be on `PATH`).
//!
//! Replaced later by in-process decoders; the CLI keeps Phase 0 buildable without FFmpeg dev libs.

use std::io::Cursor;
use std::path::Path;
use std::process::Command;

use image::ImageFormat;
use serde::{Deserialize, Serialize};

use crate::DecodeError;

/// Native dimensions of the first video stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
struct FfprobeRoot {
    streams: Vec<FfprobeStream>,
}

#[derive(Debug, Deserialize)]
struct FfprobeStream {
    width: Option<u64>,
    height: Option<u64>,
}

/// Runs `ffprobe` on `path` and returns the primary video size.
pub fn probe_video_dimensions(path: &Path) -> Result<VideoDimensions, DecodeError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "json",
        ])
        .arg(path)
        .output()?;

    if !output.status.success() {
        return Err(DecodeError::process(
            "ffprobe",
            &output.stderr,
            output.status.code(),
        ));
    }

    let parsed: FfprobeRoot = serde_json::from_slice(&output.stdout)?;
    let stream = parsed.streams.first().ok_or(DecodeError::InvalidData)?;
    let width = stream.width.ok_or(DecodeError::InvalidData)? as u32;
    let height = stream.height.ok_or(DecodeError::InvalidData)? as u32;
    if width == 0 || height == 0 {
        return Err(DecodeError::InvalidData);
    }
    Ok(VideoDimensions { width, height })
}

/// Decodes the first video frame, scales to fit `max_width`, returns PNG bytes.
///
/// `max_width` caps width (preserving aspect); dimensions are even height for encoder-friendly sizes.
pub fn first_frame_preview_png(
    path: &Path,
    max_width: u32,
) -> Result<(VideoDimensions, VideoDimensions, Vec<u8>), DecodeError> {
    let src = probe_video_dimensions(path)?;
    let (out_w, out_h) = preview_dimensions(src.width, src.height, max_width);
    let expected_len = (out_w as usize)
        .checked_mul(out_h as usize)
        .and_then(|n| n.checked_mul(3))
        .ok_or(DecodeError::InvalidData)?;

    let output = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg(path)
        .arg("-map")
        .arg("0:v:0")
        .arg("-an")
        .arg("-vframes")
        .arg("1")
        .arg("-vf")
        .arg(format!("format=rgb24,scale={out_w}:{out_h}"))
        .arg("-f")
        .arg("rawvideo")
        .arg("-pix_fmt")
        .arg("rgb24")
        .arg("-")
        .output()?;

    if !output.status.success() {
        return Err(DecodeError::process(
            "ffmpeg",
            &output.stderr,
            output.status.code(),
        ));
    }

    if output.stdout.len() != expected_len {
        return Err(DecodeError::BadFrameSize {
            got: output.stdout.len(),
            expected: expected_len,
        });
    }

    let rgb = image::RgbImage::from_raw(out_w, out_h, output.stdout).ok_or(DecodeError::InvalidData)?;
    let mut cursor = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(rgb).write_to(&mut cursor, ImageFormat::Png)?;
    let preview = VideoDimensions {
        width: out_w,
        height: out_h,
    };
    Ok((src, preview, cursor.into_inner()))
}

fn preview_dimensions(src_w: u32, src_h: u32, max_width: u32) -> (u32, u32) {
    let max_width = max_width.max(16);
    let (mut w, mut h) = if src_w <= max_width {
        (src_w, src_h)
    } else {
        (
            max_width,
            (src_h as u64 * max_width as u64 / src_w as u64) as u32,
        )
    };
    h = ((h / 2).max(1)) * 2;
    w = w.max(2);
    h = h.max(2);
    (w, h)
}

#[cfg(test)]
mod tests {
    use super::preview_dimensions;

    #[test]
    fn preview_dimensions_scales_down() {
        let (w, h) = preview_dimensions(3840, 2160, 1280);
        assert_eq!(w, 1280);
        assert_eq!(h, 720);
    }

    #[test]
    fn preview_dimensions_no_upscale() {
        let (w, h) = preview_dimensions(640, 480, 1280);
        assert_eq!(w, 640);
        assert_eq!(h, 480);
    }
}
