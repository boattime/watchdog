pub mod error;
pub mod platform;
pub mod types;

pub use error::WatchDogError;
pub use types::{CaptureRegion, Frame, PixelFormat};

#[cfg(target_os = "macos")]
pub use platform::macos::MacOSCapture as PlatformCapture;

pub fn can_capture() -> bool {
    PlatformCapture::new().is_ok()
}
