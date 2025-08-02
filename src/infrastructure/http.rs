use actix_web::{HttpRequest, HttpResponse, Result, web};
use crate::domain::video::{VideoChunk, parse_range_header, get_video_metadata, read_video_chunk, validate_range, format_content_range};
use crate::shared::config::Config;
use crate::application::services::VideoCreationAppService;
use crate::application::dto::CreateVideoRequest;

/// Extract range header from HTTP request
pub fn extract_range_header(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Range")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

/// Create HTTP response for video streaming
pub fn create_video_response(chunk: VideoChunk, content_type: &str) -> HttpResponse {
    HttpResponse::PartialContent()
        .content_type(content_type)
        .append_header(("Content-Range", format_content_range(&chunk.range)))
        .append_header(("Accept-Ranges", "bytes"))
        .body(chunk.data)
}

/// Create error response
pub fn create_error_response(status: actix_web::http::StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status)
        .content_type("text/plain")
        .body(message.to_string())
}

/// Handle video streaming request
pub async fn handle_video_stream(
    req: HttpRequest,
    config: web::Data<Config>
) -> Result<HttpResponse> {
    // Get video metadata
    let video_info = get_video_metadata(&config.video_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read video file"))?;
    
    // Extract and parse range header
    let range_header = extract_range_header(&req);
    let range = parse_range_header(range_header.as_deref(), video_info.total_size);
    
    // Validate range
    if !validate_range(&range) {
        return Ok(create_error_response(
            actix_web::http::StatusCode::RANGE_NOT_SATISFIABLE,
            "Invalid range request"
        ));
    }
    
    // Read video chunk
    let chunk = read_video_chunk(&config.video_path, &range)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read video chunk"))?;
    
    // Create response
    Ok(create_video_response(chunk, &config.content_type))
}

/// Handle video creation from images using query parameters
/// Example: POST /create-video?video_id=test&output_path=output.mp4&image1=img1.jpg&image2=img2.jpg
pub async fn handle_create_video(
    query: web::Query<std::collections::HashMap<String, String>>,
    config: web::Data<Config>,
) -> Result<HttpResponse> {
    let service = VideoCreationAppService::new(config.get_ref().clone());
    
    // Parse query parameters
    let video_id = query.get("video_id")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing video_id parameter"))?;
    
    let output_path = query.get("output_path")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing output_path parameter"))?;
    
    // Collect image paths from image1, image2, image3, etc.
    let mut image_paths = Vec::new();
    let mut i = 1;
    while let Some(image_path) = query.get(&format!("image{}", i)) {
        image_paths.push(image_path.clone());
        i += 1;
    }
    
    if image_paths.is_empty() {
        return Ok(create_error_response(
            actix_web::http::StatusCode::BAD_REQUEST,
            "No image paths provided. Use image1, image2, etc. parameters"
        ));
    }
    
    let request = CreateVideoRequest {
        video_id: video_id.clone(),
        output_path: output_path.clone(),
        image_paths,
        width: query.get("width").and_then(|w| w.parse().ok()),
        height: query.get("height").and_then(|h| h.parse().ok()),
        duration_per_image: query.get("duration").and_then(|d| d.parse().ok()),
    };
    
    match service.create_video(request) {
        Ok(response) => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("Video creation job started.\nJob ID: {}\nStatus: {}\nTotal frames: {}\nEstimated duration: {:.1}s", 
                response.job_id, response.status, response.total_frames, response.estimated_duration))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("Error creating video: {}", e))),
    }
}

/// Handle video creation job status check
pub async fn handle_get_job_status(
    path: web::Path<String>,
    config: web::Data<Config>,
) -> Result<HttpResponse> {
    let job_id = path.into_inner();
    let service = VideoCreationAppService::new(config.get_ref().clone());
    
    match service.get_job_status(&job_id) {
        Ok(response) => {
            let progress_info = if let Some(progress) = &response.progress {
                format!("\nProgress: {}/{} frames ({:.1}%)", 
                    progress.current_frame, progress.total_frames, progress.percentage)
            } else {
                String::new()
            };
            
            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(format!("Job ID: {}\nVideo ID: {}\nStatus: {}{}", 
                    response.job_id, response.video_id, response.status, progress_info)))
        },
        Err(e) => Ok(HttpResponse::NotFound()
            .content_type("text/plain")
            .body(format!("Job not found: {}", e))),
    }
}

/// Handle image validation using query parameters
/// Example: GET /validate-images?image1=img1.jpg&image2=img2.jpg
pub async fn handle_validate_images(
    query: web::Query<std::collections::HashMap<String, String>>,
    config: web::Data<Config>,
) -> Result<HttpResponse> {
    let service = VideoCreationAppService::new(config.get_ref().clone());
    
    // Collect image paths from image1, image2, image3, etc.
    let mut image_paths = Vec::new();
    let mut i = 1;
    while let Some(image_path) = query.get(&format!("image{}", i)) {
        image_paths.push(image_path.clone());
        i += 1;
    }
    
    if image_paths.is_empty() {
        return Ok(create_error_response(
            actix_web::http::StatusCode::BAD_REQUEST,
            "No image paths provided. Use image1, image2, etc. parameters"
        ));
    }
    
    match service.validate_images(&image_paths) {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("All {} images are valid", image_paths.len()))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("Validation failed: {}", e))),
    }
}

/// Health check endpoint
pub async fn handle_health_check(
    config: web::Data<Config>,
) -> Result<HttpResponse> {
    // Check if FFmpeg is available
    let ffmpeg_available = crate::infrastructure::ffmpeg::FFmpegVideoCreator::<
        crate::infrastructure::repositories::InMemoryVideoCreationRepository
    >::check_ffmpeg_available();
    
    // Validate configuration
    let config_valid = config.validate().is_ok();
    
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Video Streaming API - Health Check\nStatus: OK\nFFmpeg Available: {}\nConfig Valid: {}\nDefault Image Spec: {}x{} ({}s per image)\nTimestamp: {:?}", 
            ffmpeg_available, config_valid, config.default_image_width, config.default_image_height, config.default_duration_per_image, std::time::SystemTime::now())))
} 