use actix_web::{web, App, HttpServer};
use video_streaming_api::{config::Config, http::handle_video_stream};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env();
    let server_address = config.server_address();
    
    println!("Starting video streaming server on {}", server_address);
    println!("Serving video from: {}", config.video_path);
    
    // Create and run server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/stream", web::get().to(handle_video_stream))
    })
    .bind(server_address)?
    .run()
    .await
}
