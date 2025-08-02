use crate::domain::common::{DomainResult, DomainError, ByteRange};
use crate::domain::video::{VideoChunk, VideoId};

/// Aggregate Root: Streaming Session
#[derive(Debug, Clone)]
pub struct StreamingSession {
    pub id: SessionId,
    pub video_id: VideoId,
    pub client_info: ClientInfo,
    pub state: SessionState,
    pub metrics: SessionMetrics,
}

impl StreamingSession {
    pub fn new(id: SessionId, video_id: VideoId, client_info: ClientInfo) -> Self {
        StreamingSession {
            id,
            video_id,
            client_info,
            state: SessionState::Created,
            metrics: SessionMetrics::new(),
        }
    }

    pub fn start(&mut self) -> DomainResult<()> {
        match self.state {
            SessionState::Created => {
                self.state = SessionState::Active;
                self.metrics.start_time = std::time::SystemTime::now();
                Ok(())
            }
            _ => Err(DomainError::InvalidState("Session already started".to_string())),
        }
    }

    pub fn pause(&mut self) -> DomainResult<()> {
        match self.state {
            SessionState::Active => {
                self.state = SessionState::Paused;
                self.metrics.update_pause_time();
                Ok(())
            }
            _ => Err(DomainError::InvalidState("Session not active".to_string())),
        }
    }

    pub fn resume(&mut self) -> DomainResult<()> {
        match self.state {
            SessionState::Paused => {
                self.state = SessionState::Active;
                self.metrics.update_resume_time();
                Ok(())
            }
            _ => Err(DomainError::InvalidState("Session not paused".to_string())),
        }
    }

    pub fn end(&mut self) -> DomainResult<()> {
        self.state = SessionState::Ended;
        self.metrics.end_time = Some(std::time::SystemTime::now());
        Ok(())
    }

    pub fn request_chunk(&mut self, range: &ByteRange) -> DomainResult<VideoChunk> {
        if self.state != SessionState::Active {
            return Err(DomainError::InvalidState("Session not active".to_string()));
        }

        self.metrics.bytes_requested += range.size();
        self.metrics.chunks_requested += 1;

        // This would delegate to a domain service
        Ok(VideoChunk::new(self.video_id.clone(), range.clone(), vec![]))
    }
}

/// Entity: Session ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: String) -> Self {
        SessionId(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Value Object: Client Information
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub user_agent: String,
    pub ip_address: String,
    pub supported_formats: Vec<String>,
}

impl ClientInfo {
    pub fn new(user_agent: String, ip_address: String) -> Self {
        ClientInfo {
            user_agent,
            ip_address,
            supported_formats: vec!["video/webm".to_string(), "video/mp4".to_string()],
        }
    }
}

/// Value Object: Session State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Created,
    Active,
    Paused,
    Ended,
}

/// Value Object: Session Metrics
#[derive(Debug, Clone)]
pub struct SessionMetrics {
    pub start_time: std::time::SystemTime,
    pub end_time: Option<std::time::SystemTime>,
    pub bytes_requested: u64,
    pub chunks_requested: u64,
    pub pause_count: u32,
    pub total_pause_duration: std::time::Duration,
}

impl SessionMetrics {
    pub fn new() -> Self {
        SessionMetrics {
            start_time: std::time::SystemTime::now(),
            end_time: None,
            bytes_requested: 0,
            chunks_requested: 0,
            pause_count: 0,
            total_pause_duration: std::time::Duration::ZERO,
        }
    }

    pub fn update_pause_time(&mut self) {
        self.pause_count += 1;
        // In a real implementation, you'd track pause start time
    }

    pub fn update_resume_time(&mut self) {
        // In a real implementation, you'd calculate pause duration
        self.total_pause_duration += std::time::Duration::from_secs(1);
    }

    pub fn duration(&self) -> Option<std::time::Duration> {
        self.end_time?.duration_since(self.start_time).ok()
    }
}

/// Domain Service: Session Repository Interface
pub trait SessionRepository {
    fn find_by_id(&self, id: &SessionId) -> DomainResult<Option<StreamingSession>>;
    fn save(&self, session: &StreamingSession) -> DomainResult<()>;
    fn delete(&self, id: &SessionId) -> DomainResult<()>;
    fn find_active_sessions(&self) -> DomainResult<Vec<StreamingSession>>;
}

/// Domain Service: Session Manager
pub struct SessionManager;

impl SessionManager {
    pub fn create_session(video_id: VideoId, client_info: ClientInfo) -> StreamingSession {
        let session_id = SessionId::new(format!("session_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()));
        StreamingSession::new(session_id, video_id, client_info)
    }

    pub fn validate_session(session: &StreamingSession) -> DomainResult<()> {
        if session.state == SessionState::Ended {
            return Err(DomainError::InvalidState("Session has ended".to_string()));
        }
        Ok(())
    }
} 