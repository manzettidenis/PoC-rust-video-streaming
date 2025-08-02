use actix_web::{web, App, HttpServer};
use video_streaming_api::{
    shared::config::Config, 
    infrastructure::http::{
        handle_video_stream, handle_create_video, handle_get_job_status, 
        handle_validate_images, handle_health_check
    }
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load and validate configuration
    let config = Config::from_env();
    
    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    }
    
    let server_address = config.server_address();
    
    println!("🚀 Starting Video Streaming API PoC");
    println!("==================================");
    println!("Server: {}", server_address);
    println!("Video Path: {}", config.video_path);
    println!("Default Image Spec: {}x{} ({}s per image)", 
        config.default_image_width, config.default_image_height, config.default_duration_per_image);
    println!("FFmpeg Path: {}", config.ffmpeg_path);
    println!("FFmpeg Codec: {}", config.ffmpeg_codec);
    println!("✅ FFmpeg Available: {}", video_streaming_api::infrastructure::ffmpeg::FFmpegVideoCreator::<
        video_streaming_api::infrastructure::repositories::InMemoryVideoCreationRepository
    >::check_ffmpeg_available());
    println!("==================================");
    
    // Create and run server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            // Video streaming endpoints
            .route("/stream", web::get().to(handle_video_stream))
            // Video creation endpoints
            .route("/create-video", web::post().to(handle_create_video))
            .route("/job/{job_id}", web::get().to(handle_get_job_status))
            .route("/validate-images", web::get().to(handle_validate_images))
            // Health check
            .route("/health", web::get().to(handle_health_check))
    })
    .bind(server_address)?
    .run()
    .await
}
