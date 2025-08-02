# Assets Directory

This directory contains test assets for the video streaming API PoC.

## Structure

```
assets/
├── images/          # Test images for video creation
│   ├── sample1.jpg
│   ├── sample2.jpg
│   └── sample3.jpg
├── videos/          # Test videos for streaming
│   └── sample.webm
└── output/          # Generated videos from image processing
    └── .gitkeep
```

## Usage

### For Video Creation Testing
Place test images in `images/` folder and use them with the video creation API:

```bash
# Example: Create video from test images
curl -X POST "http://localhost:8080/create-video?video_id=test_video&output_path=assets/output/test_output.mp4&image1=assets/images/sample1.jpg&image2=assets/images/sample2.jpg&image3=assets/images/sample3.jpg"
```

### For Video Streaming Testing
Place test videos in `videos/` folder and update the `VIDEO_PATH` environment variable:

```bash
export VIDEO_PATH="assets/videos/sample.webm"
cargo run
```

## File Formats Supported

### Images (for video creation)
- JPG/JPEG
- PNG
- BMP
- TIFF
- WebP

### Videos (for streaming)
- WebM
- MP4
- AVI
- MOV

## Notes
- The `output/` directory is for generated videos and should be added to `.gitignore`
- Test images should be 800x600 or similar aspect ratio for best results
- Keep test files small for quick development iterations 