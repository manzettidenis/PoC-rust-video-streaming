use crate::domain::common::{ByteRange};
use crate::domain::video::{VideoMetadata};

/// DTO for video streaming request
#[derive(Debug)]
pub struct StreamVideoRequest {
    pub video_id: String,
    pub range_header: Option<String>,
}

/// DTO for video streaming response
#[derive(Debug)]
pub struct StreamVideoResponse {
    pub video_id: String,
    pub content_type: String,
    pub content_range: String,
    pub data: Vec<u8>,
}

/// DTO for video metadata response
#[derive(Debug)]
pub struct VideoMetadataResponse {
    pub video_id: String,
    pub total_size: u64,
    pub content_type: String,
    pub duration: Option<f64>,
    pub bitrate: Option<u32>,
}

/// DTO for session creation request
#[derive(Debug)]
pub struct CreateSessionRequest {
    pub video_id: String,
    pub user_agent: String,
    pub ip_address: String,
}

/// DTO for session response
#[derive(Debug)]
pub struct SessionResponse {
    pub session_id: String,
    pub video_id: String,
    pub state: String,
    pub metrics: SessionMetricsResponse,
}

/// DTO for session metrics
#[derive(Debug)]
pub struct SessionMetricsResponse {
    pub bytes_requested: u64,
    pub chunks_requested: u64,
    pub pause_count: u32,
    pub duration_seconds: Option<u64>,
}

/// DTO for video creation request
#[derive(Debug)]
pub struct CreateVideoRequest {
    pub image_paths: Vec<String>,
    pub output_path: String,
    pub video_id: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_per_image: Option<u32>,
}

/// DTO for video creation response
#[derive(Debug)]
pub struct CreateVideoResponse {
    pub job_id: String,
    pub video_id: String,
    pub status: String,
    pub total_frames: usize,
    pub estimated_duration: u32,
    pub created_at: String,
}

/// DTO for video creation job status
#[derive(Debug)]
pub struct VideoCreationJobStatusResponse {
    pub job_id: String,
    pub video_id: String,
    pub status: String,
    pub progress: Option<VideoCreationProgressResponse>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub error_message: Option<String>,
    pub duration_seconds: Option<u64>,
}

/// DTO for video creation progress
#[derive(Debug)]
pub struct VideoCreationProgressResponse {
    pub current_frame: usize,
    pub total_frames: usize,
    pub percentage: f32,
    pub estimated_time_remaining: Option<f32>,
}

impl From<VideoMetadata> for VideoMetadataResponse {
    fn from(metadata: VideoMetadata) -> Self {
        VideoMetadataResponse {
            video_id: "".to_string(), // Will be set by the caller
            total_size: metadata.total_size,
            content_type: metadata.content_type.as_str().to_string(),
            duration: metadata.duration,
            bitrate: metadata.bitrate,
        }
    }
}

impl From<ByteRange> for String {
    fn from(range: ByteRange) -> Self {
        format!("bytes {}-{}/{}", range.start, range.end, range.total_size)
    }
} 