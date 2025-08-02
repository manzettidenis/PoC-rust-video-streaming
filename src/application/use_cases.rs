use crate::domain::common::{DomainResult};
use crate::domain::video::{ VideoId, VideoRepository, VideoStreamingService, RangeParser};
use crate::domain::streaming::{SessionId, SessionRepository, SessionManager, ClientInfo};
use crate::application::dto::{StreamVideoRequest, StreamVideoResponse, CreateSessionRequest, SessionResponse};

/// Use Case: Stream Video
pub struct StreamVideoUseCase<R, S> 
where 
    R: VideoRepository,
    S: VideoStreamingService,
{
    video_repository: R,
    streaming_service: S,
}

impl<R, S> StreamVideoUseCase<R, S>
where 
    R: VideoRepository,
    S: VideoStreamingService,
{
    pub fn new(video_repository: R, streaming_service: S) -> Self {
        Self {
            video_repository,
            streaming_service,
        }
    }

    pub fn execute(&self, request: StreamVideoRequest) -> DomainResult<StreamVideoResponse> {
        let video_id = VideoId::new(request.video_id);
        
        // Find video
        let video = self.video_repository.find_by_id(&video_id)?
            .ok_or_else(|| crate::domain::common::DomainError::FileNotFound)?;
        
        // Parse range header
        let range = RangeParser::parse_range_header(
            request.range_header.as_deref(), 
            video.metadata.total_size
        )?;
        
        // Read video chunk
        let chunk = self.streaming_service.read_chunk(&video, &range)?;
        
        // Create response
        Ok(StreamVideoResponse {
            video_id: video_id.as_str().to_string(),
            content_type: video.metadata.content_type.as_str().to_string(),
            content_range: format!("bytes {}-{}/{}", range.start, range.end, range.total_size),
            data: chunk.data,
        })
    }
}

/// Use Case: Create Streaming Session
pub struct CreateSessionUseCase<R> 
where 
    R: SessionRepository,
{
    session_repository: R,
}

impl<R> CreateSessionUseCase<R>
where 
    R: SessionRepository,
{
    pub fn new(session_repository: R) -> Self {
        Self { session_repository }
    }

    pub fn execute(&self, request: CreateSessionRequest) -> DomainResult<SessionResponse> {
        let video_id = VideoId::new(request.video_id);
        let client_info = ClientInfo::new(request.user_agent, request.ip_address);
        
        // Create session
        let mut session = SessionManager::create_session(video_id, client_info);
        
        // Start session
        session.start()?;
        
        // Save session
        self.session_repository.save(&session)?;
        
        // Create response
        Ok(SessionResponse {
            session_id: session.id.as_str().to_string(),
            video_id: session.video_id.as_str().to_string(),
            state: format!("{:?}", session.state),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: session.metrics.bytes_requested,
                chunks_requested: session.metrics.chunks_requested,
                pause_count: session.metrics.pause_count,
                duration_seconds: session.metrics.duration().map(|d| d.as_secs()),
            },
        })
    }
}

/// Use Case: Manage Session State
pub struct ManageSessionUseCase<R> 
where 
    R: SessionRepository,
{
    session_repository: R,
}

impl<R> ManageSessionUseCase<R>
where 
    R: SessionRepository,
{
    pub fn new(session_repository: R) -> Self {
        Self { session_repository }
    }

    pub fn pause_session(&self, session_id: &str) -> DomainResult<SessionResponse> {
        let session_id = SessionId::new(session_id.to_string());
        
        let mut session = self.session_repository.find_by_id(&session_id)?
            .ok_or_else(|| crate::domain::common::DomainError::FileNotFound)?;
        
        session.pause()?;
        self.session_repository.save(&session)?;
        
        Ok(SessionResponse {
            session_id: session.id.as_str().to_string(),
            video_id: session.video_id.as_str().to_string(),
            state: format!("{:?}", session.state),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: session.metrics.bytes_requested,
                chunks_requested: session.metrics.chunks_requested,
                pause_count: session.metrics.pause_count,
                duration_seconds: session.metrics.duration().map(|d| d.as_secs()),
            },
        })
    }

    pub fn resume_session(&self, session_id: &str) -> DomainResult<SessionResponse> {
        let session_id = SessionId::new(session_id.to_string());
        
        let mut session = self.session_repository.find_by_id(&session_id)?
            .ok_or_else(|| crate::domain::common::DomainError::FileNotFound)?;
        
        session.resume()?;
        self.session_repository.save(&session)?;
        
        Ok(SessionResponse {
            session_id: session.id.as_str().to_string(),
            video_id: session.video_id.as_str().to_string(),
            state: format!("{:?}", session.state),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: session.metrics.bytes_requested,
                chunks_requested: session.metrics.chunks_requested,
                pause_count: session.metrics.pause_count,
                duration_seconds: session.metrics.duration().map(|d| d.as_secs()),
            },
        })
    }

    pub fn end_session(&self, session_id: &str) -> DomainResult<SessionResponse> {
        let session_id = SessionId::new(session_id.to_string());
        
        let mut session = self.session_repository.find_by_id(&session_id)?
            .ok_or_else(|| crate::domain::common::DomainError::FileNotFound)?;
        
        session.end()?;
        self.session_repository.save(&session)?;
        
        Ok(SessionResponse {
            session_id: session.id.as_str().to_string(),
            video_id: session.video_id.as_str().to_string(),
            state: format!("{:?}", session.state),
            metrics: crate::application::dto::SessionMetricsResponse {
                bytes_requested: session.metrics.bytes_requested,
                chunks_requested: session.metrics.chunks_requested,
                pause_count: session.metrics.pause_count,
                duration_seconds: session.metrics.duration().map(|d| d.as_secs()),
            },
        })
    }
} 