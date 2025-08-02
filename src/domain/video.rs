use std::fs::{File};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use crate::domain::common::{DomainResult, DomainError, ContentType, FilePath, ByteRange};

/// Entity: Video ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoId(String);

impl VideoId {
    pub fn new(id: String) -> Self {
        VideoId(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Value Object: Video Metadata
#[derive(Debug, Clone)]
pub struct VideoMetadata {
    pub total_size: u64,
    pub content_type: ContentType,
    pub duration: Option<f64>, // in seconds
    pub bitrate: Option<u32>,  // in bits per second
}

impl VideoMetadata {
    pub fn new(total_size: u64, content_type: ContentType) -> Self {
        VideoMetadata {
            total_size,
            content_type,
            duration: None,
            bitrate: None,
        }
    }

    pub fn from_path(file_path: &FilePath) -> DomainResult<Self> {
        let path = Path::new(file_path.as_str());
        
        if !path.exists() {
            return Err(DomainError::FileNotFound);
        }

        let metadata = std::fs::metadata(file_path.as_str())
            .map_err(|e| DomainError::IoError(e.to_string()))?;

        let content_type = Self::infer_content_type(file_path.as_str())?;

        Ok(VideoMetadata {
            total_size: metadata.len(),
            content_type,
            duration: None, // Would be extracted by a media analysis service
            bitrate: None,  // Would be extracted by a media analysis service
        })
    }

    fn infer_content_type(file_path: &str) -> DomainResult<ContentType> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let content_type = match extension {
            "webm" => "video/webm",
            "mp4" => "video/mp4",
            "avi" => "video/x-msvideo",
            "mov" => "video/quicktime",
            "mkv" => "video/x-matroska",
            _ => return Err(DomainError::InvalidContentType),
        };

        ContentType::new(content_type.to_string())
    }
}

/// Aggregate Root: Video
#[derive(Debug, Clone)]
pub struct Video {
    pub id: VideoId,
    pub metadata: VideoMetadata,
    pub file_path: FilePath,
}

impl Video {
    pub fn new(id: VideoId, file_path: FilePath) -> DomainResult<Self> {
        let metadata = VideoMetadata::from_path(&file_path)?;
        
        Ok(Video {
            id,
            metadata,
            file_path,
        })
    }

    pub fn supports_range_requests(&self) -> bool {
        true // Most video formats support range requests
    }
}

/// Video file information
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub total_size: u64,
    pub content_type: String,
}

/// Range request information
#[derive(Debug, Clone)]
pub struct RangeRequest {
    pub start: u64,
    pub end: u64,
    pub total_size: u64,
}

/// Video streaming data
#[derive(Debug)]
pub struct VideoChunk {
    pub data: Vec<u8>,
    pub range: RangeRequest,
}

impl VideoChunk {
    pub fn new(_video_id: VideoId, range: ByteRange, data: Vec<u8>) -> Self {
        VideoChunk {
            data,
            range: RangeRequest {
                start: range.start,
                end: range.end,
                total_size: range.total_size,
            },
        }
    }
}

/// Domain Service: Video Repository Interface
pub trait VideoRepository {
    fn find_by_id(&self, id: &VideoId) -> DomainResult<Option<Video>>;
    fn save(&self, video: &Video) -> DomainResult<()>;
    fn delete(&self, id: &VideoId) -> DomainResult<()>;
}

/// Domain Service: Video Streaming Service
pub trait VideoStreamingService {
    fn read_chunk(&self, video: &Video, range: &ByteRange) -> DomainResult<VideoChunk>;
    fn get_metadata(&self, video: &Video) -> DomainResult<VideoMetadata>;
}

/// Domain Service: Range Parser
pub struct RangeParser;

impl RangeParser {
    pub fn parse_range_header(range_header: Option<&str>, total_size: u64) -> DomainResult<ByteRange> {
        match range_header {
            Some(range) => {
                let parts: Vec<&str> = range.trim_start_matches("bytes=").split('-').collect();
                let start = parts.get(0).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                let end = parts.get(1)
                    .and_then(|s| if s.is_empty() { None } else { Some(s) })
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(total_size - 1);
                
                ByteRange::new(start, end.min(total_size - 1), total_size)
            }
            None => ByteRange::new(0, total_size - 1, total_size)
        }
    }
}

/// Parse range header from HTTP request
pub fn parse_range_header(range_header: Option<&str>, total_size: u64) -> RangeRequest {
    match range_header {
        Some(range) => {
            let parts: Vec<&str> = range.trim_start_matches("bytes=").split('-').collect();
            let start = parts.get(0).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let end = parts.get(1)
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(total_size - 1);
            
            RangeRequest {
                start,
                end: end.min(total_size - 1),
                total_size,
            }
        }
        None => RangeRequest {
            start: 0,
            end: total_size - 1,
            total_size,
        }
    }
}

/// Get video file metadata
pub fn get_video_metadata(file_path: &str) -> Result<VideoInfo, std::io::Error> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    
    Ok(VideoInfo {
        total_size: metadata.len(),
        content_type: infer_content_type(file_path),
    })
}

/// Read video chunk based on range request
pub fn read_video_chunk(file_path: &str, range: &RangeRequest) -> Result<VideoChunk, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    
    // Seek to start position
    reader.seek(SeekFrom::Start(range.start))?;
    
    // Calculate chunk size
    let chunk_size = (range.end - range.start + 1) as usize;
    
    // Read the chunk
    let mut buffer = vec![0u8; chunk_size];
    let bytes_read = reader.take(chunk_size as u64).read(&mut buffer)?;
    buffer.truncate(bytes_read);
    
    Ok(VideoChunk {
        data: buffer,
        range: range.clone(),
    })
}

/// Infer content type from file extension
fn infer_content_type(file_path: &str) -> String {
    match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
        Some("webm") => "video/webm".to_string(),
        Some("mp4") => "video/mp4".to_string(),
        Some("avi") => "video/x-msvideo".to_string(),
        Some("mov") => "video/quicktime".to_string(),
        Some("mkv") => "video/x-matroska".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

/// Validate range request
pub fn validate_range(range: &RangeRequest) -> bool {
    range.start <= range.end && 
    range.end < range.total_size && 
    range.start < range.total_size
}

/// Calculate content range header value
pub fn format_content_range(range: &RangeRequest) -> String {
    format!("bytes {}-{}/{}", range.start, range.end, range.total_size)
} 