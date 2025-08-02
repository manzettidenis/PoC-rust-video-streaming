use std::process::Command;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_ffmpeg_debug() {
    println!("🔧 Debugging FFmpeg Video Creation");
    println!("==================================");

    // Test 1: Check if we can create a simple video manually
    println!("🎥 Test 1: Manual FFmpeg video creation");
    
    let output_path = "assets/output/debug_test.mp4";
    
    // Create a simple test video using FFmpeg
    let output = Command::new("ffmpeg")
        .arg("-f").arg("lavfi")
        .arg("-i").arg("color=red:size=800x600:duration=3")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-y")
        .arg(output_path)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Manual FFmpeg video creation successful");
                if Path::new(output_path).exists() {
                    if let Ok(metadata) = fs::metadata(output_path) {
                        println!("   File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("❌ Manual FFmpeg video creation failed");
                println!("   Exit code: {}", output.status);
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Failed to execute FFmpeg: {}", e);
        }
    }

    // Test 2: Check our concat file creation with absolute paths
    println!("\n🎥 Test 2: Concat file creation with absolute paths");
    
    let temp_dir = std::env::temp_dir();
    let list_file = temp_dir.join("ffmpeg_debug_list.txt");
    
    // Get absolute paths
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let image1_path = current_dir.join("assets/images/test1.jpg");
    let image2_path = current_dir.join("assets/images/test2.jpg");
    let image3_path = current_dir.join("assets/images/test3.jpg");
    
    let content = format!(
        "file '{}'\nduration 1\nfile '{}'\nduration 1\nfile '{}'\n",
        image1_path.to_string_lossy(),
        image2_path.to_string_lossy(),
        image3_path.to_string_lossy()
    );
    
    match fs::write(&list_file, &content) {
        Ok(_) => {
            println!("✅ Concat file created: {}", list_file.display());
            println!("   Content:\n{}", content);
        }
        Err(e) => {
            println!("❌ Failed to create concat file: {}", e);
        }
    }

    // Test 3: Try concat video creation
    println!("\n🎥 Test 3: Concat video creation");
    
    let concat_output = "assets/output/debug_concat.mp4";
    
    let concat_cmd = Command::new("ffmpeg")
        .arg("-f").arg("concat")
        .arg("-safe").arg("0")
        .arg("-i").arg(list_file.to_str().unwrap())
        .arg("-vf").arg("scale=800:600")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-r").arg("1")
        .arg("-y")
        .arg(concat_output)
        .output();

    match concat_cmd {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Concat video creation successful");
                if Path::new(concat_output).exists() {
                    if let Ok(metadata) = fs::metadata(concat_output) {
                        println!("   File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("❌ Concat video creation failed");
                println!("   Exit code: {}", output.status);
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Failed to execute concat FFmpeg: {}", e);
        }
    }

    // Clean up
    if let Err(e) = fs::remove_file(&list_file) {
        println!("Warning: Failed to clean up temp file: {}", e);
    }
} 