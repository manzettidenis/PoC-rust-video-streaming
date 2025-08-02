use crate::domain::common::DomainResult;
use crate::domain::video::VideoId;
use crate::domain::video_creation::{VideoCreationManager, ImageSpec, VideoCreationJobId, VideoCreator, VideoCreationRepository};
use crate::application::dto::{
    StreamVideoRequest, StreamVideoResponse, CreateSessionRequest, SessionResponse,
    CreateVideoRequest, CreateVideoResponse, VideoCreationJobStatusResponse, VideoCreationProgressResponse
};
use crate::infrastructure::ffmpeg::FFmpegVideoCreator;
use crate::infrastructure::repositories::InMemoryVideoCreationRepository;

/// Simplified application service for basic operations
pub struct VideoStreamingAppService;

impl VideoStreamingAppService {
    pub fn new() -> Self {
        Self
    }

    pub fn stream_video(&self, request: StreamVideoRequest) -> DomainResult<StreamVideoResponse> {
        // For now, return a simple response
        // In a real implementation, this would use repositories and domain services
        Ok(StreamVideoResponse {
            video_id: request.video_id,
            content_type: "video/webm".to_string(),
            content_range: "bytes 0-1023/2048".to_string(),
            data: vec![0u8; 1024],
        })
    }
}

/// Simplified session management service
pub struct SessionManagementAppService;

impl SessionManagementAppService {
    pub fn new() -> Self {
        Self
    }

    pub fn create_session(&self, request: CreateSessionRequest) -> DomainResult<SessionResponse> {
        // For now, return a simple response
        // In a real implementation, this would use repositories and domain services
        Ok(SessionResponse {
            session_id: "session_123".to_string(),
            video_id: request.video_id,
            state: "Active".to_string(),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: 0,
                chunks_requested: 0,
                pause_count: 0,
                duration_seconds: None,
            },
        })
    }

    pub fn pause_session(&self, session_id: &str) -> DomainResult<SessionResponse> {
        Ok(SessionResponse {
            session_id: session_id.to_string(),
            video_id: "video_123".to_string(),
            state: "Paused".to_string(),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: 1024,
                chunks_requested: 1,
                pause_count: 1,
                duration_seconds: Some(30),
            },
        })
    }

    pub fn resume_session(&self, session_id: &str) -> DomainResult<SessionResponse> {
        Ok(SessionResponse {
            session_id: session_id.to_string(),
            video_id: "video_123".to_string(),
            state: "Active".to_string(),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: 1024,
                chunks_requested: 1,
                pause_count: 1,
                duration_seconds: Some(45),
            },
        })
    }

    pub fn end_session(&self, session_id: &str) -> DomainResult<SessionResponse> {
        Ok(SessionResponse {
            session_id: session_id.to_string(),
            video_id: "video_123".to_string(),
            state: "Ended".to_string(),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: 2048,
                chunks_requested: 2,
                pause_count: 1,
                duration_seconds: Some(120),
            },
        })
    }
}

/// Video creation application service
pub struct VideoCreationAppService {
    config: crate::shared::config::Config,
    repository: InMemoryVideoCreationRepository,
    ffmpeg_creator: FFmpegVideoCreator<InMemoryVideoCreationRepository>,
}

impl VideoCreationAppService {
    pub fn new(config: crate::shared::config::Config) -> Self {
        let repository = InMemoryVideoCreationRepository::new();
        let ffmpeg_creator = FFmpegVideoCreator::new(repository.clone());
        
        Self { 
            config,
            repository,
            ffmpeg_creator,
        }
    }

    pub fn create_video(&self, request: CreateVideoRequest) -> DomainResult<CreateVideoResponse> {
        // Parse video ID
        let video_id = VideoId::new(request.video_id.clone());

        // Create image specification from request or use config defaults
        let image_spec = if let (Some(width), Some(height), Some(duration)) = 
            (request.width, request.height, request.duration_per_image) {
            Some(ImageSpec::new(width, height, duration)?)
        } else {
            Some(self.config.default_image_spec())
        };

        // Create domain job using the domain service
        let job = VideoCreationManager::create_job(
            request.image_paths.clone(),
            request.output_path.clone(),
            video_id,
            image_spec,
        )?;

        // Save the job to repository
        self.repository.save_job(&job)?;

        // Start the job
        let mut job = job;
        job.start()?;
        self.repository.update_job(&job)?;

        // Actually create the video using FFmpeg
        match self.ffmpeg_creator.create_video(&job.request) {
            Ok(completed_job) => {
                // Update the job with final status
                self.repository.update_job(&completed_job)?;
                
                Ok(CreateVideoResponse {
                    job_id: completed_job.id.as_str().to_string(),
                    video_id: request.video_id,
                    status: format!("{:?}", completed_job.status),
                    total_frames: completed_job.request.frame_count(),
                    estimated_duration: completed_job.request.total_duration(),
                    created_at: format!("{:?}", completed_job.created_at),
                })
            }
            Err(e) => {
                // Mark job as failed
                job.fail(e.to_string())?;
                self.repository.update_job(&job)?;
                
                Err(e)
            }
        }
    }

    pub fn get_job_status(&self, job_id: &str) -> DomainResult<VideoCreationJobStatusResponse> {
        let job_id = VideoCreationJobId::new(job_id.to_string());
        
        match self.repository.find_job_by_id(&job_id)? {
            Some(job) => {
                let progress = job.progress.as_ref().map(|p| VideoCreationProgressResponse {
                    current_frame: p.current_frame,
                    total_frames: p.total_frames,
                    percentage: p.percentage,
                    estimated_time_remaining: p.estimated_time_remaining_seconds,
                });

                Ok(VideoCreationJobStatusResponse {
                    job_id: job.id.as_str().to_string(),
                    video_id: job.request.video_id.as_str().to_string(),
                    status: format!("{:?}", job.status),
                    progress,
                    created_at: format!("{:?}", job.created_at),
                    completed_at: job.completed_at.map(|t| format!("{:?}", t)),
                    error_message: job.error_message.clone(),
                    duration_seconds: job.duration().map(|d| d.as_secs() as u64),
                })
            }
            None => Err(crate::domain::common::DomainError::FileNotFound)
        }
    }

    pub fn validate_images(&self, image_paths: &[String]) -> DomainResult<bool> {
        // Convert to FilePath and validate using domain service
        let file_paths: Vec<crate::domain::common::FilePath> = image_paths
            .iter()
            .map(|p| crate::domain::common::FilePath::new(p.clone()))
            .collect();

        VideoCreationManager::validate_images(&file_paths)?;
        Ok(true)
    }
} 