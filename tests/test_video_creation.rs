use std::path::Path;
use video_streaming_api::{
    domain::video_creation::{VideoCreationManager, ImageSpec, VideoCreationJobId, VideoCreator},
    domain::video::VideoId,
    domain::common::FilePath,
    shared::config::Config,
    infrastructure::ffmpeg::FFmpegVideoCreator,
    infrastructure::repositories::InMemoryVideoCreationRepository,
};

/// Test function to create a video from images in assets/images/
#[tokio::test]
async fn test_create_video_from_assets() {
    println!("🎬 Testing Video Creation from Assets");
    println!("=====================================");

    // Load configuration
    let config = Config::from_env();
    println!("📋 Configuration loaded:");
    println!("   - Default image spec: {}x{} ({}s per image)", 
        config.default_image_width, config.default_image_height, config.default_duration_per_image);
    println!("   - FFmpeg path: {}", config.ffmpeg_path);
    println!("   - FFmpeg codec: {}", config.ffmpeg_codec);

    // Check if FFmpeg is available
    let ffmpeg_available = FFmpegVideoCreator::<InMemoryVideoCreationRepository>::check_ffmpeg_available();
    println!("✅ FFmpeg available: {}", ffmpeg_available);

    if !ffmpeg_available {
        println!("❌ FFmpeg not available. Skipping video creation test.");
        return;
    }

    // Get image files from assets/images/
    let images_dir = Path::new("assets/images");
    if !images_dir.exists() {
        println!("❌ Assets/images directory not found. Creating it...");
        std::fs::create_dir_all(images_dir).expect("Failed to create assets/images directory");
        println!("📁 Created assets/images directory");
        println!("   Please add some test images (jpg, png, etc.) to assets/images/ and run the test again");
        return;
    }

    let image_files = get_image_files(images_dir);
    if image_files.is_empty() {
        println!("❌ No image files found in assets/images/");
        println!("   Please add some test images (jpg, png, etc.) to assets/images/ and run the test again");
        return;
    }

    println!("📸 Found {} image files:", image_files.len());
    for (i, file) in image_files.iter().enumerate() {
        println!("   {}. {}", i + 1, file);
    }

    // Create output directory
    let output_dir = Path::new("assets/output");
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).expect("Failed to create assets/output directory");
        println!("📁 Created assets/output directory");
    }

    // Test 1: Create video with default settings
    println!("\n🎥 Test 1: Creating video with default settings");
    test_video_creation(&image_files, "assets/output/test_default.mp4", &config, None).await;

    // Test 2: Create video with custom settings
    println!("\n🎥 Test 2: Creating video with custom settings");
    let custom_spec = ImageSpec::new(640, 480, 2).expect("Invalid image spec");
    test_video_creation(&image_files, "assets/output/test_custom.mp4", &config, Some(custom_spec)).await;

    // Test 3: Create video with high quality settings
    println!("\n🎥 Test 3: Creating video with high quality settings");
    let hq_spec = ImageSpec::new(1920, 1080, 1).expect("Invalid image spec");
    test_video_creation(&image_files, "assets/output/test_hq.mp4", &config, Some(hq_spec)).await;

    println!("\n✅ All tests completed!");
    println!("📁 Check assets/output/ for generated videos");
}

/// Helper function to get image files from a directory
fn get_image_files(dir: &Path) -> Vec<String> {
    let mut image_files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        match ext_str.to_lowercase().as_str() {
                            "jpg" | "jpeg" | "png" | "bmp" | "tiff" | "webp" => {
                                if let Some(file_name) = path.to_str() {
                                    image_files.push(file_name.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    // Sort files for consistent ordering
    image_files.sort();
    image_files
}

/// Test video creation with given parameters
async fn test_video_creation(
    image_files: &[String], 
    output_path: &str, 
    config: &Config, 
    custom_spec: Option<ImageSpec>
) {
    let video_id = VideoId::new(format!("test_video_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()));

    println!("   📹 Video ID: {}", video_id.as_str());
    println!("   📁 Output: {}", output_path);
    
    if let Some(spec) = &custom_spec {
        println!("   🎨 Custom spec: {}x{} ({}s per image)", 
            spec.width, spec.height, spec.duration_seconds);
    } else {
        println!("   🎨 Using default spec: {}x{} ({}s per image)", 
            config.default_image_width, config.default_image_height, config.default_duration_per_image);
    }

    // Create job using domain service
    match VideoCreationManager::create_job(
        image_files.to_vec(),
        output_path.to_string(),
        video_id.clone(),
        custom_spec,
    ) {
        Ok(job) => {
            println!("   ✅ Job created successfully:");
            println!("      - Job ID: {}", job.id.as_str());
            println!("      - Total frames: {}", job.request.frame_count());
            println!("      - Estimated duration: {:.1}s", job.request.total_duration());
            println!("      - Status: {:?}", job.status);

            // Validate images
            match VideoCreationManager::validate_images(&job.request.image_paths) {
                Ok(_) => println!("   ✅ Image validation passed"),
                Err(e) => println!("   ❌ Image validation failed: {}", e),
            }

            // Try to create the video using FFmpeg
            let repository = InMemoryVideoCreationRepository::new();
            let ffmpeg_creator = FFmpegVideoCreator::new(repository);
            
            match ffmpeg_creator.create_video(&job.request) {
                Ok(completed_job) => {
                    println!("   ✅ Video creation completed:");
                    println!("      - Final status: {:?}", completed_job.status);
                    if let Some(duration) = completed_job.duration() {
                        println!("      - Processing time: {:.2}s", duration.as_secs_f32());
                    }
                    if let Some(error) = &completed_job.error_message {
                        println!("      - Error: {}", error);
                    }
                    
                    // Check if output file exists
                    if Path::new(output_path).exists() {
                        if let Ok(metadata) = std::fs::metadata(output_path) {
                            println!("      - Output file size: {} bytes", metadata.len());
                        }
                        println!("   🎉 Video file created successfully!");
                    } else {
                        println!("   ❌ Output file not found");
                    }
                }
                Err(e) => {
                    println!("   ❌ Video creation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to create job: {}", e);
        }
    }
}

/// Test function to validate images only
#[tokio::test]
async fn test_validate_images() {
    println!("🔍 Testing Image Validation");
    println!("============================");

    let images_dir = Path::new("assets/images");
    if !images_dir.exists() {
        println!("❌ Assets/images directory not found");
        return;
    }

    let image_files = get_image_files(images_dir);
    if image_files.is_empty() {
        println!("❌ No image files found in assets/images/");
        return;
    }

    println!("📸 Validating {} image files:", image_files.len());
    
    // Convert to FilePath objects
    let file_paths: Vec<FilePath> = image_files
        .iter()
        .map(|p| FilePath::new(p.clone()))
        .collect();

    match VideoCreationManager::validate_images(&file_paths) {
        Ok(_) => {
            println!("✅ All images are valid!");
            for (i, path) in file_paths.iter().enumerate() {
                println!("   {}. ✅ {}", i + 1, path.as_str());
            }
        }
        Err(e) => {
            println!("❌ Image validation failed: {}", e);
        }
    }
}

/// Test function to check FFmpeg availability
#[tokio::test]
async fn test_ffmpeg_availability() {
    println!("🔧 Testing FFmpeg Availability");
    println!("==============================");

    let ffmpeg_available = FFmpegVideoCreator::<InMemoryVideoCreationRepository>::check_ffmpeg_available();
    
    if ffmpeg_available {
        println!("✅ FFmpeg is available on the system");
        
        // Test FFmpeg version
        let output = std::process::Command::new("ffmpeg")
            .arg("-version")
            .output();
            
        match output {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    if let Some(first_line) = version.lines().next() {
                        println!("   Version: {}", first_line);
                    }
                }
            }
            Err(e) => {
                println!("   Warning: Could not get FFmpeg version: {}", e);
            }
        }
    } else {
        println!("❌ FFmpeg is not available on the system");
        println!("   Please install FFmpeg to use video creation features");
        println!("   Ubuntu/Debian: sudo apt install ffmpeg");
        println!("   macOS: brew install ffmpeg");
        println!("   Windows: Download from https://ffmpeg.org/download.html");
    }
} 