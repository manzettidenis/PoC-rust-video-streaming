#!/bin/bash

# Script to create a video directly using our domain services
# This bypasses the API and uses our Rust code directly

echo "🎬 Creating Video Directly from Assets"
echo "======================================"

# Check if we have images
if [ ! -d "assets/images" ]; then
    echo "❌ assets/images directory not found"
    exit 1
fi

# Count images
IMAGE_COUNT=$(ls assets/images/*.{jpg,jpeg,png,webp} 2>/dev/null | wc -l)

if [ $IMAGE_COUNT -eq 0 ]; then
    echo "❌ No images found in assets/images/"
    echo "   Please run ./scripts/generate_test_images.sh first"
    exit 1
fi

echo "📸 Found $IMAGE_COUNT images in assets/images/"
echo ""

# Create output directory
mkdir -p assets/output

# Run our Rust test that creates videos directly
echo "🎥 Creating video using our domain services..."
echo "   - Resolution: 800x600 (default)"
echo "   - Duration per image: 1 second"
echo "   - Total duration: $IMAGE_COUNT seconds"
echo ""

# Run the test that creates videos
cargo test test_create_video_from_assets -- --nocapture

echo ""
echo "✅ Video creation completed!"
echo "📁 Check assets/output/ for generated videos:"
ls -lh assets/output/*.mp4 2>/dev/null || echo "No MP4 files found"

echo ""
echo "🎉 Process completed!" 