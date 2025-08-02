use std::env;

/// Configuration for the video streaming service
#[derive(Debug, Clone)]
pub struct Config {
    pub video_path: String,
    pub host: String,
    pub port: u16,
    pub content_type: String,
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self {
            video_path: "videos/dg.webm".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            content_type: "video/webm".to_string(),
        }
    }

    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::new();
        
        if let Ok(path) = env::var("VIDEO_PATH") {
            config.video_path = path;
        }
        
        if let Ok(host) = env::var("HOST") {
            config.host = host;
        }
        
        if let Ok(port) = env::var("PORT") {
            if let Ok(port_num) = port.parse() {
                config.port = port_num;
            }
        }
        
        if let Ok(content_type) = env::var("CONTENT_TYPE") {
            config.content_type = content_type;
        }
        
        config
    }

    /// Get the full server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
} 