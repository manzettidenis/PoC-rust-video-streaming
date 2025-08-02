use std::process::Command;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_ffmpeg_sequence_debug() {
    println!("🔧 FFmpeg Sequence Debug Test");
    println!("=============================");

    // Test with 3 distinct images
    let images = vec![
        "assets/images/test1.jpg",  // Should be first
        "assets/images/test2.jpg",  // Should be second  
        "assets/images/test3.jpg",  // Should be third
    ];

    println!("📸 Testing with {} images:", images.len());
    for (i, img) in images.iter().enumerate() {
        println!("   {}. {}", i + 1, img);
    }

    // Approach 1: Current implementation (buggy - repeats first image)
    println!("\n🎥 Approach 1: Current Implementation (buggy)");
    let output1 = "assets/output/sequence_buggy.mp4";
    let mut cmd1 = Command::new("ffmpeg");
    
    // Add each image as a separate input with loop
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        cmd1.arg("-loop").arg("1")
            .arg("-t").arg("1")
            .arg("-i").arg(&absolute_path);
    }
    
    // Build filter complex
    let filter_complex = format!("[0:v][1:v][2:v]concat=n=3:v=1:a=0,scale=800:600[outv]");
    
    let cmd1_result = cmd1
        .arg("-filter_complex").arg(&filter_complex)
        .arg("-map").arg("[outv]")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-r").arg("1")
        .arg("-y")
        .arg(output1)
        .output();

    match cmd1_result {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Current approach successful");
                if Path::new(output1).exists() {
                    if let Ok(metadata) = fs::metadata(output1) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Current approach failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Current approach error: {}", e);
        }
    }

    // Approach 2: Using concat demuxer with proper duration
    println!("\n🎥 Approach 2: Concat demuxer with duration");
    let temp_dir = std::env::temp_dir();
    let list_file = temp_dir.join("ffmpeg_sequence_concat.txt");
    
    let mut content = String::new();
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        content.push_str(&format!("file '{}'\n", absolute_path));
        content.push_str("duration 1\n");
    }
    // Add the last image again to ensure proper duration
    let last_absolute_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(images.last().unwrap())
        .to_string_lossy()
        .to_string();
    content.push_str(&format!("file '{}'\n", last_absolute_path));
    
    fs::write(&list_file, &content).expect("Failed to write concat file");
    println!("   Concat file content:");
    println!("{}", content);
    
    let output2 = "assets/output/sequence_concat.mp4";
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
                println!("   ✅ Concat demuxer successful");
                if Path::new(output2).exists() {
                    if let Ok(metadata) = fs::metadata(output2) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Concat demuxer failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Concat demuxer error: {}", e);
        }
    }

    // Approach 3: Using individual image inputs without loop
    println!("\n🎥 Approach 3: Individual inputs without loop");
    let output3 = "assets/output/sequence_individual.mp4";
    let mut cmd3 = Command::new("ffmpeg");
    
    // Add each image as a separate input without loop
    for image_path in &images {
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(image_path)
            .to_string_lossy()
            .to_string();
        
        cmd3.arg("-i").arg(&absolute_path);
    }
    
    // Build filter complex with fps filter to control duration
    let filter_complex = format!("[0:v][1:v][2:v]concat=n=3:v=1:a=0,fps=1,scale=800:600[outv]");
    
    let cmd3_result = cmd3
        .arg("-filter_complex").arg(&filter_complex)
        .arg("-map").arg("[outv]")
        .arg("-c:v").arg("libx264")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-y")
        .arg(output3)
        .output();

    match cmd3_result {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ Individual inputs successful");
                if Path::new(output3).exists() {
                    if let Ok(metadata) = fs::metadata(output3) {
                        println!("   📊 File size: {} bytes", metadata.len());
                    }
                }
            } else {
                println!("   ❌ Individual inputs failed");
                println!("   Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("   ❌ Individual inputs error: {}", e);
        }
    }

    // Clean up
    if let Err(e) = fs::remove_file(&list_file) {
        println!("Warning: Failed to clean up temp file: {}", e);
    }

    // Check durations and analyze results
    println!("\n📊 Analysis of generated videos:");
    for i in 1..=3 {
        let output_file = match i {
            1 => "assets/output/sequence_buggy.mp4",
            2 => "assets/output/sequence_concat.mp4", 
            3 => "assets/output/sequence_individual.mp4",
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