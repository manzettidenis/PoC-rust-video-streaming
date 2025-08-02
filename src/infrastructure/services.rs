// Infrastructure layer service implementations
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::domain::video::{Video, VideoStreamingService, VideoChunk, VideoMetadata};
use crate::domain::common::{DomainResult, DomainError, ByteRange};

/// File-based video streaming service implementation
pub struct FileVideoStreamingService;

impl FileVideoStreamingService {
    pub fn new() -> Self {
        Self
    }
}

impl VideoStreamingService for FileVideoStreamingService {
    fn read_chunk(&self, video: &Video, range: &ByteRange) -> DomainResult<VideoChunk> {
        let file = File::open(video.file_path.as_str())
            .map_err(|e| DomainError::IoError(e.to_string()))?;
        
        let mut reader = BufReader::new(file);
        
        // Seek to start position
        reader.seek(SeekFrom::Start(range.start))
            .map_err(|e| DomainError::IoError(e.to_string()))?;
        
        // Calculate chunk size
        let chunk_size = range.size() as usize;
        
        // Read the chunk
        let mut buffer = vec![0u8; chunk_size];
        let bytes_read = reader.take(chunk_size as u64).read(&mut buffer)
            .map_err(|e| DomainError::IoError(e.to_string()))?;
        buffer.truncate(bytes_read);
        
        Ok(VideoChunk::new(video.id.clone(), range.clone(), buffer))
    }

    fn get_metadata(&self, video: &Video) -> DomainResult<VideoMetadata> {
        Ok(video.metadata.clone())
    }
} 