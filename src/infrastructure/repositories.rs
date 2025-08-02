// Infrastructure layer repository implementations
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use crate::domain::video::{Video, VideoId, VideoRepository};
use crate::domain::streaming::{StreamingSession, SessionId, SessionRepository, SessionState};
use crate::domain::video_creation::{VideoCreationJob, VideoCreationJobId, VideoCreationRepository, VideoCreationStatus};
use crate::domain::common::DomainResult;

/// In-memory video repository implementation
pub struct InMemoryVideoRepository {
    videos: Mutex<HashMap<VideoId, Video>>,
}

impl InMemoryVideoRepository {
    pub fn new() -> Self {
        Self {
            videos: Mutex::new(HashMap::new()),
        }
    }
}

impl VideoRepository for InMemoryVideoRepository {
    fn find_by_id(&self, id: &VideoId) -> DomainResult<Option<Video>> {
        let videos = self.videos.lock().unwrap();
        Ok(videos.get(id).cloned())
    }

    fn save(&self, video: &Video) -> DomainResult<()> {
        let mut videos = self.videos.lock().unwrap();
        videos.insert(video.id.clone(), video.clone());
        Ok(())
    }

    fn delete(&self, id: &VideoId) -> DomainResult<()> {
        let mut videos = self.videos.lock().unwrap();
        videos.remove(id);
        Ok(())
    }
}

/// In-memory session repository implementation
pub struct InMemorySessionRepository {
    sessions: Mutex<HashMap<SessionId, StreamingSession>>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

impl SessionRepository for InMemorySessionRepository {
    fn find_by_id(&self, id: &SessionId) -> DomainResult<Option<StreamingSession>> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.get(id).cloned())
    }

    fn save(&self, session: &StreamingSession) -> DomainResult<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.id.clone(), session.clone());
        Ok(())
    }

    fn delete(&self, id: &SessionId) -> DomainResult<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(id);
        Ok(())
    }

    fn find_active_sessions(&self) -> DomainResult<Vec<StreamingSession>> {
        let sessions = self.sessions.lock().unwrap();
        let active_sessions: Vec<StreamingSession> = sessions.values()
            .filter(|s| matches!(s.state, SessionState::Active))
            .cloned()
            .collect();
        Ok(active_sessions)
    }
}

/// In-memory video creation job repository implementation
#[derive(Clone)]
pub struct InMemoryVideoCreationRepository {
    jobs: Arc<Mutex<HashMap<VideoCreationJobId, VideoCreationJob>>>,
}

impl InMemoryVideoCreationRepository {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl VideoCreationRepository for InMemoryVideoCreationRepository {
    fn save_job(&self, job: &VideoCreationJob) -> DomainResult<()> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(job.id.clone(), job.clone());
        Ok(())
    }

    fn find_job_by_id(&self, id: &VideoCreationJobId) -> DomainResult<Option<VideoCreationJob>> {
        let jobs = self.jobs.lock().unwrap();
        Ok(jobs.get(id).cloned())
    }

    fn find_jobs_by_status(&self, status: &VideoCreationStatus) -> DomainResult<Vec<VideoCreationJob>> {
        let jobs = self.jobs.lock().unwrap();
        let filtered_jobs: Vec<VideoCreationJob> = jobs.values()
            .filter(|j| &j.status == status)
            .cloned()
            .collect();
        Ok(filtered_jobs)
    }

    fn update_job(&self, job: &VideoCreationJob) -> DomainResult<()> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(job.id.clone(), job.clone());
        Ok(())
    }
} 