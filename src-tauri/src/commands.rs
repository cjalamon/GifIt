//! Tauri commands exposed to the React frontend via `invoke(...)`.

use crate::capture;

#[tauri::command]
pub fn start_capture() -> Result<(), String> {
    capture::start_capture()
}

#[tauri::command]
pub fn stop_capture() -> Result<(), String> {
    capture::stop_capture().map(|_session| ())
}

#[tauri::command]
pub fn export_gif(output_path: String) -> Result<(), String> {
    // TODO: pull real frames from the last capture session and pass them,
    // plus user-chosen EncodeSettings, into encode::encode_frames_to_gif.
    let _ = output_path;
    todo!("Wire captured frames through encode::encode_frames_to_gif")
}
