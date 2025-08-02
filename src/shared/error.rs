use std::fmt;

/// Custom error types for the video streaming service
#[derive(Debug)]
pub enum VideoStreamError {
    IoError(std::io::Error),
    InvalidRange,
    FileNotFound,
    InvalidContentType,
}

impl fmt::Display for VideoStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VideoStreamError::IoError(err) => write!(f, "IO error: {}", err),
            VideoStreamError::InvalidRange => write!(f, "Invalid range request"),
            VideoStreamError::FileNotFound => write!(f, "Video file not found"),
            VideoStreamError::InvalidContentType => write!(f, "Invalid content type"),
        }
    }
}

impl std::error::Error for VideoStreamError {}

impl From<std::io::Error> for VideoStreamError {
    fn from(err: std::io::Error) -> Self {
        VideoStreamError::IoError(err)
    }
}

/// Result type for video streaming operations
pub type VideoResult<T> = Result<T, VideoStreamError>;

/// Convert IO error to video stream error
pub fn map_io_error<T>(result: std::io::Result<T>) -> VideoResult<T> {
    result.map_err(VideoStreamError::from)
}

/// Validate file path exists
pub fn validate_file_path(path: &str) -> VideoResult<()> {
    if std::path::Path::new(path).exists() {
        Ok(())
    } else {
        Err(VideoStreamError::FileNotFound)
    }
} 