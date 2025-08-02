use std::fs::{File, Metadata};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

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