use std::fmt;

/// Domain error types
#[derive(Debug, Clone)]
pub enum DomainError {
    InvalidRange,
    FileNotFound,
    InvalidContentType,
    InvalidState(String),
    IoError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidRange => write!(f, "Invalid range request"),
            DomainError::FileNotFound => write!(f, "File not found"),
            DomainError::InvalidContentType => write!(f, "Invalid content type"),
            DomainError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            DomainError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

/// Result type for domain operations
pub type DomainResult<T> = Result<T, DomainError>;

/// Value Object: Content Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentType(String);

impl ContentType {
    pub fn new(content_type: String) -> DomainResult<Self> {
        match content_type.as_str() {
            "video/webm" | "video/mp4" | "video/x-msvideo" | 
            "video/quicktime" | "video/x-matroska" => Ok(ContentType(content_type)),
            _ => Err(DomainError::InvalidContentType),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Value Object: File Path
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePath(String);

impl FilePath {
    pub fn new(path: String) -> Self {
        FilePath(path)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Value Object: Byte Range
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteRange {
    pub start: u64,
    pub end: u64,
    pub total_size: u64,
}

impl ByteRange {
    pub fn new(start: u64, end: u64, total_size: u64) -> DomainResult<Self> {
        if start > end || end >= total_size || start >= total_size {
            return Err(DomainError::InvalidRange);
        }
        
        Ok(ByteRange {
            start,
            end,
            total_size,
        })
    }

    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn is_valid(&self) -> bool {
        self.start <= self.end && 
        self.end < self.total_size && 
        self.start < self.total_size
    }
} 