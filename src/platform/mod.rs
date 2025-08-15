#[cfg(target_os = "macos")]
pub mod macos;

// Re-export the platform-specific implementation
#[cfg(target_os = "macos")]
pub use macos::MacOSCapture;

// For other platforms, we'll add them later
#[cfg(not(target_os = "macos"))]
compile_error!("WatchDog currently only supports macOS");
