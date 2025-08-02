# Video Streaming API - PoC

A proof-of-concept video streaming API built with Rust and Actix Web, following Domain-Driven Design principles.

## Features

- HTTP range request support for video streaming
- **FFmpeg integration for video creation from images**
- Domain-driven architecture with clear layer separation
- In-memory session management
- Configurable via environment variables
- **Image-to-video conversion with customizable settings**

## Quick Start

### Prerequisites
- Rust 1.70+
- FFmpeg installed on your system
- Test images and videos (see `assets/` folder)

### Setup
```bash
# Clone the repository
git clone <repo-url>
cd video_streaming_api

# Copy environment file and customize
cp .env.example .env

# Create assets directory structure
mkdir -p assets/{images,videos,output}

# Add test images to assets/images/
# Add test videos to assets/videos/

# Run the server
cargo run
```

### Usage Examples

#### Video Streaming
```bash
# Stream video with range requests
curl -H "Range: bytes=0-1023" http://localhost:8080/stream

# Stream without range (full file)
curl http://localhost:8080/stream
```

#### Video Creation from Images
```bash
# Create video from 3 images (1 second each)
curl -X POST "http://localhost:8080/create-video?video_id=my_video&output_path=assets/output/test.mp4&image1=assets/images/img1.jpg&image2=assets/images/img2.jpg&image3=assets/images/img3.jpg"

# Create video with custom settings
curl -X POST "http://localhost:8080/create-video?video_id=custom_video&output_path=assets/output/custom.mp4&image1=assets/images/img1.jpg&image2=assets/images/img2.jpg&width=1920&height=1080&duration=2.0"

# Check job status
curl "http://localhost:8080/job/job_1234567890"

# Validate images before processing
curl "http://localhost:8080/validate-images?image1=assets/images/img1.jpg&image2=assets/images/img2.jpg"

# Health check (includes FFmpeg availability)
curl "http://localhost:8080/health"
```

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and customize:

```bash
# Video streaming configuration
VIDEO_PATH=assets/videos/sample.webm
CONTENT_TYPE=video/webm

# Server configuration
HOST=127.0.0.1
PORT=8080

# Video creation configuration
DEFAULT_IMAGE_WIDTH=800
DEFAULT_IMAGE_HEIGHT=600
DEFAULT_DURATION_PER_IMAGE=1.0

# FFmpeg configuration
FFMPEG_PATH=ffmpeg
FFMPEG_CODEC=libx264
FFMPEG_PIXEL_FORMAT=yuv420p

# Development configuration
RUST_LOG=info
RUST_BACKTRACE=1
```

### Assets Structure

```
assets/
├── images/          # Test images for video creation
│   ├── sample1.jpg
│   ├── sample2.jpg
│   └── sample3.jpg
├── videos/          # Test videos for streaming
│   └── sample.webm
└── output/          # Generated videos (gitignored)
    └── .gitkeep
```

## Architecture

```
src/
├── domain/           # Business logic and entities
│   ├── video.rs      # Video streaming domain
│   ├── video_creation.rs  # Video creation domain
│   ├── streaming.rs  # Session management
│   └── common.rs     # Shared domain types
├── application/      # Use cases and DTOs
│   ├── services.rs   # Application services
│   ├── use_cases.rs  # Use case implementations
│   └── dto.rs        # Data transfer objects
├── infrastructure/   # HTTP, repositories, external services
│   ├── http.rs       # HTTP handlers
│   ├── ffmpeg.rs     # FFmpeg integration
│   ├── repositories.rs # In-memory repositories
│   └── services.rs   # Infrastructure services
├── shared/           # Configuration and error handling
│   ├── config.rs     # Environment configuration
│   └── error.rs      # Error types
└── main.rs           # Application entry point
```

## API Endpoints

### Video Streaming
- `GET /stream` - Stream video content with optional range requests

### Video Creation
- `POST /create-video` - Create video from images
- `GET /job/{job_id}` - Check video creation job status
- `GET /validate-images` - Validate image files

### System
- `GET /health` - Health check and system status

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Run with logs
RUST_LOG=debug cargo run

# Check FFmpeg availability
curl http://localhost:8080/health
```

## PoC Limitations

- Single video file support for streaming
- In-memory storage only
- Basic error handling
- No authentication
- No video metadata extraction
- **Synchronous FFmpeg processing (no background jobs)**

## Next Steps

- Multiple video support
- Database persistence
- User authentication
- Video transcoding
- CDN integration
- **Background job processing with Redis/PostgreSQL**
- **Real-time progress updates via WebSockets**
- **Video thumbnail generation** 