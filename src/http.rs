use actix_web::{HttpRequest, HttpResponse, Result, web};
use crate::video::{ VideoChunk, parse_range_header, get_video_metadata, read_video_chunk, validate_range, format_content_range};
use crate::config::Config;

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