use std::env;

/// Configuration for the video streaming service
#[derive(Debug, Clone)]
pub struct Config {
    // Video streaming configuration
    pub video_path: String,
    pub content_type: String,
    
    // Server configuration
    pub host: String,
    pub port: u16,
    
    // Video creation configuration
    pub default_image_width: u32,
    pub default_image_height: u32,
    pub default_duration_per_image: u32,
    
    // FFmpeg configuration
    pub ffmpeg_path: String,
    pub ffmpeg_codec: String,
    pub ffmpeg_pixel_format: String,
    
    // Development configuration
    pub rust_log: String,
    pub rust_backtrace: String,
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self {
            video_path: "assets/videos/sample.webm".to_string(),
            content_type: "video/webm".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            default_image_width: 800,
            default_image_height: 600,
            default_duration_per_image: 1,
            ffmpeg_path: "ffmpeg".to_string(),
            ffmpeg_codec: "libx264".to_string(),
            ffmpeg_pixel_format: "yuv420p".to_string(),
            rust_log: "info".to_string(),
            rust_backtrace: "1".to_string(),
        }
    }

    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Config {
            // Video streaming configuration
            video_path: env::var("VIDEO_PATH").unwrap_or_else(|_| "assets/videos/sample.webm".to_string()),
            content_type: env::var("CONTENT_TYPE").unwrap_or_else(|_| "video/webm".to_string()),
            
            // Server configuration
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            
            // Video creation configuration
            default_image_width: env::var("DEFAULT_IMAGE_WIDTH")
                .unwrap_or_else(|_| "800".to_string())
                .parse()
                .expect("DEFAULT_IMAGE_WIDTH must be a valid number"),
            default_image_height: env::var("DEFAULT_IMAGE_HEIGHT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .expect("DEFAULT_IMAGE_HEIGHT must be a valid number"),
            default_duration_per_image: env::var("DEFAULT_DURATION_PER_IMAGE")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("DEFAULT_DURATION_PER_IMAGE must be a valid number"),
            
            // FFmpeg configuration
            ffmpeg_path: env::var("FFMPEG_PATH").unwrap_or_else(|_| "ffmpeg".to_string()),
            ffmpeg_codec: env::var("FFMPEG_CODEC").unwrap_or_else(|_| "libx264".to_string()),
            ffmpeg_pixel_format: env::var("FFMPEG_PIXEL_FORMAT").unwrap_or_else(|_| "yuv420p".to_string()),
            
            // Development configuration
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            rust_backtrace: env::var("RUST_BACKTRACE").unwrap_or_else(|_| "1".to_string()),
        }
    }

    /// Get the full server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Get default image specification
    pub fn default_image_spec(&self) -> crate::domain::video_creation::ImageSpec {
        crate::domain::video_creation::ImageSpec {
            width: self.default_image_width,
            height: self.default_image_height,
            duration_seconds: self.default_duration_per_image,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("PORT cannot be 0".to_string());
        }
        
        if self.default_image_width == 0 || self.default_image_height == 0 {
            return Err("Image dimensions cannot be 0".to_string());
        }
        
        if self.default_duration_per_image <= 0 {
            return Err("Duration per image must be positive".to_string());
        }
        
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
} 