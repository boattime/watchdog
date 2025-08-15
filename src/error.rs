#[derive(Debug, thiserror::Error)]
pub enum WatchDogError {
    #[error("Screen capture failed: {0}")]
    CaptureFailed(String),

    #[error("Permission denied: Screen recording permission required")]
    PermissionDenied,
}

impl From<String> for WatchDogError {
    fn from(msg: String) -> Self {
        WatchDogError::CaptureFailed(msg)
    }
}

impl From<&str> for WatchDogError {
    fn from(msg: &str) -> Self {
        WatchDogError::CaptureFailed(msg.to_string())
    }
}
