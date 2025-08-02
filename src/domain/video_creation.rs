use std::path::Path;
use crate::domain::common::{DomainResult, DomainError, FilePath};
use crate::domain::video::VideoId;

/// Value Object: Image specification
#[derive(Debug, Clone)]
pub struct ImageSpec {
    pub width: u32,
    pub height: u32,
    pub duration_seconds: u32,
}

impl ImageSpec {
    pub fn new(width: u32, height: u32, duration_seconds: u32) -> DomainResult<Self> {
        if width == 0 || height == 0 || duration_seconds <= 0 {
            return Err(DomainError::InvalidRange);
        }
        
        Ok(ImageSpec {
            width,
            height,
            duration_seconds,
        })
    }
}

impl Default for ImageSpec {
    fn default() -> Self {
        ImageSpec {
            width: 800,
            height: 600,
            duration_seconds: 1,
        }
    }
}

/// Value Object: Video creation request
#[derive(Debug, Clone)]
pub struct VideoCreationRequest {
    pub image_paths: Vec<FilePath>,
    pub output_path: FilePath,
    pub image_spec: ImageSpec,
    pub video_id: VideoId,
}

impl VideoCreationRequest {
    pub fn new(
        image_paths: Vec<FilePath>,
        output_path: FilePath,
        image_spec: ImageSpec,
        video_id: VideoId,
    ) -> DomainResult<Self> {
        if image_paths.is_empty() {
            return Err(DomainError::InvalidState("No images provided".to_string()));
        }

        // Validate all image paths exist
        for image_path in &image_paths {
            if !Path::new(image_path.as_str()).exists() {
                return Err(DomainError::FileNotFound);
            }
        }

        Ok(VideoCreationRequest {
            image_paths,
            output_path,
            image_spec,
            video_id,
        })
    }

    pub fn total_duration(&self) -> u32 {
        self.image_paths.len() as u32 * self.image_spec.duration_seconds
    }

    pub fn frame_count(&self) -> usize {
        self.image_paths.len()
    }
}

/// Value Object: Video creation progress
#[derive(Debug, Clone)]
pub struct VideoCreationProgress {
    pub current_frame: usize,
    pub total_frames: usize,
    pub percentage: f32,
    pub estimated_time_remaining_seconds: Option<f32>,
}

impl VideoCreationProgress {
    pub fn new(current_frame: usize, total_frames: usize) -> Self {
        let percentage = if total_frames > 0 {
            (current_frame as f32 / total_frames as f32) * 100.0
        } else {
            0.0
        };

        VideoCreationProgress {
            current_frame,
            total_frames,
            percentage,
            estimated_time_remaining_seconds: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_frame >= self.total_frames
    }
}

/// Entity: Video Creation Job
#[derive(Debug, Clone)]
pub struct VideoCreationJob {
    pub id: VideoCreationJobId,
    pub request: VideoCreationRequest,
    pub status: VideoCreationStatus,
    pub progress: Option<VideoCreationProgress>,
    pub created_at: std::time::SystemTime,
    pub completed_at: Option<std::time::SystemTime>,
    pub error_message: Option<String>,
}

impl VideoCreationJob {
    pub fn new(id: VideoCreationJobId, request: VideoCreationRequest) -> Self {
        VideoCreationJob {
            id,
            request,
            status: VideoCreationStatus::Pending,
            progress: None,
            created_at: std::time::SystemTime::now(),
            completed_at: None,
            error_message: None,
        }
    }

    pub fn start(&mut self) -> DomainResult<()> {
        match self.status {
            VideoCreationStatus::Pending => {
                self.status = VideoCreationStatus::InProgress;
                self.progress = Some(VideoCreationProgress::new(0, self.request.frame_count()));
                Ok(())
            }
            _ => Err(DomainError::InvalidState("Job already started".to_string())),
        }
    }

    pub fn update_progress(&mut self, current_frame: usize) -> DomainResult<()> {
        match self.status {
            VideoCreationStatus::InProgress => {
                self.progress = Some(VideoCreationProgress::new(current_frame, self.request.frame_count()));
                Ok(())
            }
            _ => Err(DomainError::InvalidState("Job not in progress".to_string())),
        }
    }

    pub fn complete(&mut self) -> DomainResult<()> {
        match self.status {
            VideoCreationStatus::InProgress => {
                self.status = VideoCreationStatus::Completed;
                self.completed_at = Some(std::time::SystemTime::now());
                self.progress = Some(VideoCreationProgress::new(self.request.frame_count(), self.request.frame_count()));
                Ok(())
            }
            _ => Err(DomainError::InvalidState("Job not in progress".to_string())),
        }
    }

    pub fn fail(&mut self, error_message: String) -> DomainResult<()> {
        self.status = VideoCreationStatus::Failed;
        self.error_message = Some(error_message);
        self.completed_at = Some(std::time::SystemTime::now());
        Ok(())
    }

    pub fn duration(&self) -> Option<std::time::Duration> {
        self.completed_at?.duration_since(self.created_at).ok()
    }
}

/// Entity: Video Creation Job ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoCreationJobId(String);

impl VideoCreationJobId {
    pub fn new(id: String) -> Self {
        VideoCreationJobId(id)
    }

    pub fn generate() -> Self {
        VideoCreationJobId(format!("job_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Value Object: Video Creation Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoCreationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Domain Service: Video Creator Interface
pub trait VideoCreator {
    fn create_video(&self, request: &VideoCreationRequest) -> DomainResult<VideoCreationJob>;
    fn get_job_status(&self, job_id: &VideoCreationJobId) -> DomainResult<Option<VideoCreationJob>>;
}

/// Domain Service: Video Creation Repository Interface
pub trait VideoCreationRepository {
    fn save_job(&self, job: &VideoCreationJob) -> DomainResult<()>;
    fn find_job_by_id(&self, id: &VideoCreationJobId) -> DomainResult<Option<VideoCreationJob>>;
    fn find_jobs_by_status(&self, status: &VideoCreationStatus) -> DomainResult<Vec<VideoCreationJob>>;
    fn update_job(&self, job: &VideoCreationJob) -> DomainResult<()>;
}

/// Domain Service: Video Creation Manager
pub struct VideoCreationManager;

impl VideoCreationManager {
    pub fn create_job(
        image_paths: Vec<String>,
        output_path: String,
        video_id: VideoId,
        image_spec: Option<ImageSpec>,
    ) -> DomainResult<VideoCreationJob> {
        // Convert strings to FilePath value objects
        let image_file_paths: Vec<FilePath> = image_paths
            .into_iter()
            .map(FilePath::new)
            .collect();

        let output_file_path = FilePath::new(output_path);
        let spec = image_spec.unwrap_or_default(); // Use provided spec or default (800x600, 1s)
        
        let request = VideoCreationRequest::new(
            image_file_paths,
            output_file_path,
            spec,
            video_id,
        )?;

        let job_id = VideoCreationJobId::generate();
        Ok(VideoCreationJob::new(job_id, request))
    }

    pub fn validate_images(image_paths: &[FilePath]) -> DomainResult<()> {
        for path in image_paths {
            let file_path = Path::new(path.as_str());
            
            if !file_path.exists() {
                return Err(DomainError::FileNotFound);
            }

            // Check if it's a valid image extension
            if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
                match extension.to_lowercase().as_str() {
                    "jpg" | "jpeg" | "png" | "bmp" | "tiff" | "webp" => continue,
                    _ => return Err(DomainError::InvalidContentType),
                }
            } else {
                return Err(DomainError::InvalidContentType);
            }
        }
        Ok(())
    }
} 