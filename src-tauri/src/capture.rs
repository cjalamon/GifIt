//! Screen/window capture via the Windows.Graphics.Capture API.
//!
//! Starting point for implementation: pick a target (monitor or window handle),
//! create a `GraphicsCaptureItem` for it, wire up a `Direct3D11CaptureFramePool`,
//! and copy each delivered frame's texture into an RGBA buffer here.
//! Docs: https://microsoft.github.io/windows-docs-rs/doc/windows/Graphics/Capture/index.html

pub struct CapturedFrame {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

pub struct CaptureSession {
    pub frames: Vec<CapturedFrame>,
}

pub fn start_capture() -> Result<(), String> {
    todo!("Enumerate windows/monitors, pick a target, and start a GraphicsCaptureSession")
}

pub fn stop_capture() -> Result<CaptureSession, String> {
    todo!("Stop the active capture session and return the collected frames")
}
