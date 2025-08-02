#!/bin/bash

# Script to generate test images for video creation testing
# Requires ImageMagick (convert command) or similar image generation tool

echo "🎨 Generating test images for video creation..."

# Create assets/images directory if it doesn't exist
mkdir -p assets/images

# Check if ImageMagick is available
if command -v convert &> /dev/null; then
    echo "✅ ImageMagick found, generating test images..."
    
    # Generate 5 test images with different colors and text
    convert -size 800x600 xc:red -gravity center -pointsize 60 -fill white -annotate 0 "Frame 1" assets/images/test1.jpg
    convert -size 800x600 xc:blue -gravity center -pointsize 60 -fill white -annotate 0 "Frame 2" assets/images/test2.jpg
    convert -size 800x600 xc:green -gravity center -pointsize 60 -fill white -annotate 0 "Frame 3" assets/images/test3.jpg
    convert -size 800x600 xc:yellow -gravity center -pointsize 60 -fill black -annotate 0 "Frame 4" assets/images/test4.jpg
    convert -size 800x600 xc:purple -gravity center -pointsize 60 -fill white -annotate 0 "Frame 5" assets/images/test5.jpg
    
    echo "✅ Generated 5 test images in assets/images/"
    ls -la assets/images/
    
elif command -v ffmpeg &> /dev/null; then
    echo "✅ FFmpeg found, generating test images..."
    
    # Generate test images using FFmpeg
    ffmpeg -f lavfi -i "color=red:size=800x600:duration=1" -frames:v 1 assets/images/test1.jpg -y
    ffmpeg -f lavfi -i "color=blue:size=800x600:duration=1" -frames:v 1 assets/images/test2.jpg -y
    ffmpeg -f lavfi -i "color=green:size=800x600:duration=1" -frames:v 1 assets/images/test3.jpg -y
    ffmpeg -f lavfi -i "color=yellow:size=800x600:duration=1" -frames:v 1 assets/images/test4.jpg -y
    ffmpeg -f lavfi -i "color=purple:size=800x600:duration=1" -frames:v 1 assets/images/test5.jpg -y
    
    echo "✅ Generated 5 test images in assets/images/"
    ls -la assets/images/
    
else
    echo "❌ Neither ImageMagick nor FFmpeg found for image generation"
    echo "Please install one of the following:"
    echo "  - ImageMagick: sudo apt install imagemagick"
    echo "  - FFmpeg: sudo apt install ffmpeg"
    echo ""
    echo "Or manually add some test images (jpg, png) to assets/images/"
fi 