use std::process::Command;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_ffmpeg_concat_debug() {
    println!("🔧 Debugging FFmpeg Concat File Issue");
    println!("=====================================");

    // Test with our actual images
    let images = vec![
        "assets/images/test1.jpg",
        "assets/images/test2.jpg", 
        "assets/images/test3.jpg",
        "assets/images/test4.jpg",
        "assets/images/test5.jpg",
    ];

    println!("📸 Testing with {} images:", images.len());
    for (i, img) in images.iter().enumerate() {
        println!("   {}. {}", i + 1, img);
    }

    // Create concat file with different approaches
    let temp_dir = std::env::temp_dir();
    let list_file = temp_dir.join("ffmpeg_concat_debug.txt");
    
    // Approach 1: Current implementation (buggy)
    println!("\n🎥 Approach 1: Current Implementation");
    let mut content1 = String::new();
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        content1.push_str(&format!("file '{}'\n", absolute_path));
        content1.push_str("duration 1\n");
    }
    // Add last image again
    let last_absolute_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(images.last().unwrap())
        .to_string_lossy()
        .to_string();
    content1.push_str(&format!("file '{}'\n", last_absolute_path));
    
    fs::write(&list_file, &content1).expect("Failed to write concat file");
    println!("   Concat file content:");
    println!("{}", content1);
    
    // Test approach 1
    let output1 = "assets/output/debug_approach1.mp4";
    let cmd1 = Command::new("ffmpeg")
        .arg("-f").arg("concat")
        .arg("-safe").arg("0")
        .arg("-i").arg(list_file.to_str().unwrap())
        .arg("-vf").arg("scale=800:600")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-r").arg("1")
        .arg("-y")
        .arg(output1)
        .output();

    match cmd1 {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Approach 1 successful");
                if Path::new(output1).exists() {
                    if let Ok(metadata) = fs::metadata(output1) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Approach 1 failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Approach 1 error: {}", e);
        }
    }

    // Approach 2: Fixed implementation (no duration lines)
    println!("\n🎥 Approach 2: Fixed Implementation (no duration lines)");
    let mut content2 = String::new();
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        content2.push_str(&format!("file '{}'\n", absolute_path));
    }
    
    fs::write(&list_file, &content2).expect("Failed to write concat file");
    println!("   Concat file content:");
    println!("{}", content2);
    
    // Test approach 2
    let output2 = "assets/output/debug_approach2.mp4";
    let cmd2 = Command::new("ffmpeg")
        .arg("-f").arg("concat")
        .arg("-safe").arg("0")
        .arg("-i").arg(list_file.to_str().unwrap())
        .arg("-vf").arg("scale=800:600")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-r").arg("1")
        .arg("-y")
        .arg(output2)
        .output();

    match cmd2 {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Approach 2 successful");
                if Path::new(output2).exists() {
                    if let Ok(metadata) = fs::metadata(output2) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Approach 2 failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Approach 2 error: {}", e);
        }
    }

    // Approach 3: Using loop filter
    println!("\n🎥 Approach 3: Using loop filter");
    let output3 = "assets/output/debug_approach3.mp4";
    let cmd3 = Command::new("ffmpeg")
        .arg("-loop").arg("1")
        .arg("-t").arg("1")
        .arg("-i").arg("assets/images/test1.jpg")
        .arg("-loop").arg("1")
        .arg("-t").arg("1")
        .arg("-i").arg("assets/images/test2.jpg")
        .arg("-loop").arg("1")
        .arg("-t").arg("1")
        .arg("-i").arg("assets/images/test3.jpg")
        .arg("-loop").arg("1")
        .arg("-t").arg("1")
        .arg("-i").arg("assets/images/test4.jpg")
        .arg("-loop").arg("1")
        .arg("-t").arg("1")
        .arg("-i").arg("assets/images/test5.jpg")
        .arg("-filter_complex").arg("[0:v][1:v][2:v][3:v][4:v]concat=n=5:v=1:a=0[outv]")
        .arg("-map").arg("[outv]")
        .arg("-vf").arg("scale=800:600")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-r").arg("1")
        .arg("-y")
        .arg(output3)
        .output();

    match cmd3 {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Approach 3 successful");
                if Path::new(output3).exists() {
                    if let Ok(metadata) = fs::metadata(output3) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Approach 3 failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Approach 3 error: {}", e);
        }
    }

    // Clean up
    if let Err(e) = fs::remove_file(&list_file) {
        println!("Warning: Failed to clean up temp file: {}", e);
    }

    println!("\n📊 Summary of generated files:");
    for i in 1..=3 {
        let output_file = format!("assets/output/debug_approach{}.mp4", i);
        if Path::new(&output_file).exists() {
            if let Ok(metadata) = fs::metadata(&output_file) {
                println!("   approach{}.mp4: {} bytes", i, metadata.len());
            }
        } else {
            println!("   approach{}.mp4: not created", i);
        }
    }
} 