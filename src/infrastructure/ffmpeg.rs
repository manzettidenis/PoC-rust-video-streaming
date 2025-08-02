// Infrastructure layer FFmpeg implementation
use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use crate::domain::video_creation::{
    VideoCreator, VideoCreationRequest, VideoCreationJob, VideoCreationJobId, VideoCreationRepository
};
use crate::domain::common::{DomainResult, DomainError};

/// FFmpeg-based video creator implementation
pub struct FFmpegVideoCreator<R> 
where 
    R: VideoCreationRepository,
{
    repository: R,
}

impl<R> FFmpegVideoCreator<R>
where 
    R: VideoCreationRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Check if FFmpeg is available on the system
    pub fn check_ffmpeg_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    /// Create a temporary file list for FFmpeg
    fn create_file_list(&self, request: &VideoCreationRequest) -> DomainResult<String> {
        let temp_dir = std::env::temp_dir();
        let list_file = temp_dir.join(format!("ffmpeg_list_{}.txt", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));

        let mut content = String::new();
        for image_path in &request.image_paths {
            // Use absolute paths to avoid path resolution issues
            let absolute_path = if Path::new(image_path.as_str()).is_absolute() {
                image_path.as_str().to_string()
            } else {
                std::env::current_dir()
                    .map_err(|e| DomainError::IoError(e.to_string()))?
                    .join(image_path.as_str())
                    .to_string_lossy()
                    .to_string()
            };
            
            // FFmpeg concat format: file path and duration
            content.push_str(&format!("file '{}'\n", absolute_path));
            content.push_str(&format!("duration {}\n", request.image_spec.duration_seconds));
        }
        
        // Add the last image again to ensure proper duration
        if let Some(last_image) = request.image_paths.last() {
            let absolute_path = if Path::new(last_image.as_str()).is_absolute() {
                last_image.as_str().to_string()
            } else {
                std::env::current_dir()
                    .map_err(|e| DomainError::IoError(e.to_string()))?
                    .join(last_image.as_str())
                    .to_string_lossy()
                    .to_string()
            };
            content.push_str(&format!("file '{}'\n", absolute_path));
        }

        fs::write(&list_file, content)
            .map_err(|e| DomainError::IoError(e.to_string()))?;

        Ok(list_file.to_string_lossy().to_string())
    }

    /// Execute FFmpeg command to create video
    fn execute_ffmpeg(&self, list_file: &str, request: &VideoCreationRequest) -> DomainResult<()> {
        // Use absolute path for output file as well
        let output_path = if Path::new(request.output_path.as_str()).is_absolute() {
            request.output_path.as_str().to_string()
        } else {
            std::env::current_dir()
                .map_err(|e| DomainError::IoError(e.to_string()))?
                .join(request.output_path.as_str())
                .to_string_lossy()
                .to_string()
        };

        // Use concat demuxer approach (better for image sequences)
        let output = Command::new("ffmpeg")
            .arg("-f").arg("concat")
            .arg("-safe").arg("0")
            .arg("-i").arg(list_file)
            .arg("-vf").arg(format!("scale={}:{}", request.image_spec.width, request.image_spec.height))
            .arg("-c:v").arg("libx264")
            .arg("-pix_fmt").arg("yuv420p")
            .arg("-r").arg("1") // 1 fps for consistent timing
            .arg("-y") // Overwrite output file
            .arg(&output_path)
            .output()
            .map_err(|e| DomainError::IoError(format!("Failed to execute FFmpeg: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(DomainError::IoError(format!("FFmpeg failed: {}", error_msg)));
        }

        Ok(())
    }

    /// Clean up temporary files
    fn cleanup(&self, list_file: &str) {
        if let Err(e) = fs::remove_file(list_file) {
            eprintln!("Warning: Failed to clean up temporary file {}: {}", list_file, e);
        }
    }
}

impl<R> VideoCreator for FFmpegVideoCreator<R>
where 
    R: VideoCreationRepository,
{
    fn create_video(&self, request: &VideoCreationRequest) -> DomainResult<VideoCreationJob> {
        // Check if FFmpeg is available
        if !Self::check_ffmpeg_available() {
            return Err(DomainError::IoError("FFmpeg not found on system".to_string()));
        }

        // Create a new job
        let job_id = VideoCreationJobId::generate();
        let mut job = VideoCreationJob::new(job_id, request.clone());

        // Save initial job state
        self.repository.save_job(&job)?;

        // Start the job
        job.start()?;
        self.repository.update_job(&job)?;

        // Create temporary file list
        let list_file = self.create_file_list(request)?;

        // Execute FFmpeg in a separate thread simulation (in real app, use tokio or thread pool)
        match self.execute_ffmpeg(&list_file, request) {
            Ok(_) => {
                job.complete()?;
                self.repository.update_job(&job)?;
            }
            Err(e) => {
                job.fail(e.to_string())?;
                self.repository.update_job(&job)?;
            }
        }

        // Clean up
        self.cleanup(&list_file);

        // Verify output file was created
        let output_path = if Path::new(request.output_path.as_str()).is_absolute() {
            request.output_path.as_str().to_string()
        } else {
            std::env::current_dir()
                .map_err(|e| DomainError::IoError(e.to_string()))?
                .join(request.output_path.as_str())
                .to_string_lossy()
                .to_string()
        };

        if !Path::new(&output_path).exists() {
            job.fail("Output video file was not created".to_string())?;
            self.repository.update_job(&job)?;
        }

        Ok(job)
    }

    fn get_job_status(&self, job_id: &VideoCreationJobId) -> DomainResult<Option<VideoCreationJob>> {
        self.repository.find_job_by_id(job_id)
    }
}

/// FFmpeg command builder for advanced operations
pub struct FFmpegCommandBuilder {
    command: Command,
}

impl FFmpegCommandBuilder {
    pub fn new() -> Self {
        Self {
            command: Command::new("ffmpeg"),
        }
    }

    pub fn input_concat_file(mut self, file_path: &str) -> Self {
        self.command.arg("-f").arg("concat").arg("-safe").arg("0").arg("-i").arg(file_path);
        self
    }

    pub fn scale(mut self, width: u32, height: u32) -> Self {
        self.command.arg("-vf").arg(format!("scale={}:{}", width, height));
        self
    }

    pub fn codec(mut self, codec: &str) -> Self {
        self.command.arg("-c:v").arg(codec);
        self
    }

    pub fn pixel_format(mut self, format: &str) -> Self {
        self.command.arg("-pix_fmt").arg(format);
        self
    }

    pub fn framerate(mut self, fps: u32) -> Self {
        self.command.arg("-r").arg(fps.to_string());
        self
    }

    pub fn overwrite(mut self) -> Self {
        self.command.arg("-y");
        self
    }

    pub fn output(mut self, path: &str) -> Self {
        self.command.arg(path);
        self
    }

    pub fn execute(mut self) -> DomainResult<()> {
        let output = self.command.output()
            .map_err(|e| DomainError::IoError(format!("Failed to execute FFmpeg: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(DomainError::IoError(format!("FFmpeg failed: {}", error_msg)));
        }

        Ok(())
    }
}

impl Default for FFmpegCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
} 