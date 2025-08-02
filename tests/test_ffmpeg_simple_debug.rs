use std::process::Command;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_ffmpeg_simple_debug() {
    println!("🔧 Simple FFmpeg Debug Test");
    println!("===========================");

    // Test with just 3 images to simplify
    let images = vec![
        "assets/images/test1.jpg",
        "assets/images/test2.jpg", 
        "assets/images/test3.jpg",
    ];

    println!("📸 Testing with {} images:", images.len());
    for (i, img) in images.iter().enumerate() {
        println!("   {}. {}", i + 1, img);
    }

    // Approach 1: Simple concat demuxer (no duration lines)
    println!("\n🎥 Approach 1: Simple concat demuxer");
    let temp_dir = std::env::temp_dir();
    let list_file = temp_dir.join("ffmpeg_simple_concat.txt");
    
    let mut content = String::new();
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        content.push_str(&format!("file '{}'\n", absolute_path));
    }
    
    fs::write(&list_file, &content).expect("Failed to write concat file");
    println!("   Concat file content:");
    println!("{}", content);
    
    let output1 = "assets/output/simple_concat.mp4";
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
                println!("   ✅ Simple concat successful");
                if Path::new(output1).exists() {
                    if let Ok(metadata) = fs::metadata(output1) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Simple concat failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Simple concat error: {}", e);
        }
    }

    // Approach 2: Using framerate to control duration
    println!("\n🎥 Approach 2: Using framerate control");
    let output2 = "assets/output/framerate_control.mp4";
    let cmd2 = Command::new("ffmpeg")
        .arg("-f").arg("concat")
        .arg("-safe").arg("0")
        .arg("-i").arg(list_file.to_str().unwrap())
        .arg("-vf").arg("scale=800:600,fps=1")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-y")
        .arg(output2)
        .output();

    match cmd2 {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Framerate control successful");
                if Path::new(output2).exists() {
                    if let Ok(metadata) = fs::metadata(output2) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Framerate control failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Framerate control error: {}", e);
        }
    }

    // Approach 3: Manual loop with individual inputs
    println!("\n🎥 Approach 3: Manual loop with individual inputs");
    let output3 = "assets/output/manual_loop.mp4";
    let mut cmd3 = Command::new("ffmpeg");
    
    // Add each image as a separate input with loop
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        cmd3.arg("-loop").arg("1")
            .arg("-t").arg("1")
            .arg("-i").arg(&absolute_path);
    }
    
    // Build filter complex
    let filter_complex = format!("[0:v][1:v][2:v]concat=n=3:v=1:a=0,scale=800:600[outv]");
    
    let cmd3_result = cmd3
        .arg("-filter_complex").arg(&filter_complex)
        .arg("-map").arg("[outv]")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-r").arg("1")
        .arg("-y")
        .arg(output3)
        .output();

    match cmd3_result {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Manual loop successful");
                if Path::new(output3).exists() {
                    if let Ok(metadata) = fs::metadata(output3) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Manual loop failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Manual loop error: {}", e);
        }
    }

    // Clean up
    if let Err(e) = fs::remove_file(&list_file) {
        println!("Warning: Failed to clean up temp file: {}", e);
    }

    // Check durations
    println!("\n📊 Checking video durations:");
    for i in 1..=3 {
        let output_file = match i {
            1 => "assets/output/simple_concat.mp4",
            2 => "assets/output/framerate_control.mp4", 
            3 => "assets/output/manual_loop.mp4",
            _ => continue,
        };
        
        if Path::new(output_file).exists() {
            let duration = Command::new("ffprobe")
                .arg("-v").arg("quiet")
                .arg("-show_entries").arg("format=duration")
                .arg("-of").arg("csv=p=0")
                .arg(output_file)
                .output();
                
            match duration {
                Ok(output) => {
                    if output.status.success() {
                        let duration_str = String::from_utf8_lossy(&output.stdout);
                        let duration_trimmed = duration_str.trim();
                        println!("   approach{}: {} seconds (should be 3)", i, duration_trimmed);
                    } else {
                        println!("   approach{}: could not get duration", i);
                    }
                }
                Err(_) => {
                    println!("   approach{}: could not get duration", i);
                }
            }
        } else {
            println!("   approach{}: file not created", i);
        }
    }
} 