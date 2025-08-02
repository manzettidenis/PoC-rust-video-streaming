#!/bin/bash

# Script to create a video from images in assets/images/
# This script maps the available images and creates a video using our API

echo "🎬 Creating Video from Assets Images"
echo "===================================="

# Check if server is running
if ! curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "❌ Server is not running. Please start the server with: cargo run"
    exit 1
fi

# Map the images in assets/images/
echo "📸 Mapping images in assets/images/..."

# Get all image files
IMAGE_FILES=($(ls assets/images/*.{jpg,jpeg,png,webp} 2>/dev/null | sort))

if [ ${#IMAGE_FILES[@]} -eq 0 ]; then
    echo "❌ No image files found in assets/images/"
    echo "   Please run ./scripts/generate_test_images.sh first"
    exit 1
fi

echo "✅ Found ${#IMAGE_FILES[@]} images:"
for i in "${!IMAGE_FILES[@]}"; do
    echo "   $((i+1)). ${IMAGE_FILES[$i]}"
done

# Create output directory
mkdir -p assets/output

# Create video using all images with default settings
echo ""
echo "🎥 Creating video with all images (default settings)..."
echo "   - Resolution: 800x600"
echo "   - Duration per image: 1 second"
echo "   - Total duration: ${#IMAGE_FILES[@]} seconds"

# Build the API call with all images
API_URL="http://localhost:8080/create-video"
PARAMS="video_id=assets_video_$(date +%s)&output_path=assets/output/assets_video.mp4"

# Add image parameters
for i in "${!IMAGE_FILES[@]}"; do
    PARAMS="$PARAMS&image$((i+1))=${IMAGE_FILES[$i]}"
done

# Make the API call
echo "📡 Calling API: $API_URL"
echo "   Parameters: $PARAMS"

RESPONSE=$(curl -s -X POST "$API_URL?$PARAMS")

echo ""
echo "📋 API Response:"
echo "$RESPONSE"

# Check if video was created
if [ -f "assets/output/assets_video.mp4" ]; then
    echo ""
    echo "✅ Video created successfully!"
    echo "📁 Output: assets/output/assets_video.mp4"
    echo "📊 File size: $(ls -lh assets/output/assets_video.mp4 | awk '{print $5}')"
    echo "🎬 Duration: ${#IMAGE_FILES[@]} seconds"
else
    echo ""
    echo "❌ Video file not found. Check the API response above for errors."
fi

echo ""
echo "🎉 Process completed!" 