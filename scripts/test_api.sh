#!/bin/bash

# Test script for Video Streaming API endpoints
# Make sure the server is running: cargo run

echo "🚀 Testing Video Streaming API Endpoints"
echo "========================================"

# Base URL
BASE_URL="http://localhost:8080"

# Test 1: Health check
echo "🔍 Test 1: Health Check"
echo "----------------------"
curl -s "$BASE_URL/health" | head -10
echo ""

# Test 2: Validate images
echo "🔍 Test 2: Validate Images"
echo "-------------------------"
curl -s "$BASE_URL/validate-images?image1=assets/images/test1.jpg&image2=assets/images/test2.jpg&image3=assets/images/test3.jpg"
echo ""
echo ""

# Test 3: Create video with default settings
echo "🎥 Test 3: Create Video (Default Settings)"
echo "-----------------------------------------"
curl -X POST "$BASE_URL/create-video?video_id=api_test_1&output_path=assets/output/api_test_1.mp4&image1=assets/images/test1.jpg&image2=assets/images/test2.jpg&image3=assets/images/test3.jpg"
echo ""
echo ""

# Test 4: Create video with custom settings
echo "🎥 Test 4: Create Video (Custom Settings)"
echo "----------------------------------------"
curl -X POST "$BASE_URL/create-video?video_id=api_test_2&output_path=assets/output/api_test_2.mp4&image1=assets/images/test1.jpg&image2=assets/images/test2.jpg&image3=assets/images/test3.jpg&width=640&height=480&duration=2.0"
echo ""
echo ""

# Test 5: Create video with high quality settings
echo "🎥 Test 5: Create Video (High Quality)"
echo "-------------------------------------"
curl -X POST "$BASE_URL/create-video?video_id=api_test_3&output_path=assets/output/api_test_3.mp4&image1=assets/images/test1.jpg&image2=assets/images/test2.jpg&image3=assets/images/test3.jpg&image4=assets/images/test4.jpg&image5=assets/images/test5.jpg&width=1920&height=1080&duration=1.5"
echo ""
echo ""

# Test 6: Check job status (this will be a mock response for now)
echo "📊 Test 6: Check Job Status"
echo "--------------------------"
curl -s "$BASE_URL/job/job_1234567890"
echo ""
echo ""

echo "✅ API Tests Completed!"
echo "📁 Check assets/output/ for generated videos"
echo ""
echo "Generated files:"
ls -la assets/output/*.mp4 2>/dev/null || echo "No MP4 files found" 