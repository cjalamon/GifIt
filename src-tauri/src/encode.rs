//! GIF encoding via the `gifski` crate.
//!
//! Starting point for implementation: create a `gifski::new(Settings)` collector/writer
//! pair, push each frame in on the collector thread (`add_frame_rgba`), and drive the
//! writer (`write`) on a second thread to produce the output file.
//! Docs: https://docs.rs/gifski

use imgref::ImgVec;
use rgb::RGBA8;

pub struct EncodeSettings {
    pub fps: u32,
    pub width: Option<usize>,
    pub quality: u8, // 1-100
}

pub fn encode_frames_to_gif(
    _frames: Vec<ImgVec<RGBA8>>,
    _settings: EncodeSettings,
    _output_path: &str,
) -> Result<(), String> {
    todo!("Feed frames into a gifski Collector + Writer pair and write to output_path")
}
