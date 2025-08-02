# Video Streaming API - Functional Architecture

This is a modular, pure functional video streaming API built with Rust and Actix Web, following functional programming principles.

## Architecture Overview

The codebase is organized into distinct modules, each with a single responsibility and pure functions:

### 1. **Config Module** (`src/config.rs`)
- **Purpose**: Configuration management
- **Principles**: Immutability, environment-based configuration
- **Key Functions**:
  - `Config::new()` - Creates default configuration
  - `Config::from_env()` - Loads configuration from environment variables
  - `Config::server_address()` - Pure function to generate server address

### 2. **Video Module** (`src/video.rs`)
- **Purpose**: Video file operations and range parsing
- **Principles**: Pure functions, immutable data structures
- **Key Functions**:
  - `parse_range_header()` - Pure function for parsing HTTP range headers
  - `get_video_metadata()` - Pure function for file metadata
  - `read_video_chunk()` - Pure function for reading video chunks
  - `validate_range()` - Pure function for range validation
  - `format_content_range()` - Pure function for HTTP header formatting

### 3. **HTTP Module** (`src/http.rs`)
- **Purpose**: HTTP request/response handling
- **Principles**: Pure functions, separation of concerns
- **Key Functions**:
  - `extract_range_header()` - Pure function for header extraction
  - `create_video_response()` - Pure function for response creation
  - `create_error_response()` - Pure function for error responses
  - `handle_video_stream()` - Main request handler (minimal side effects)
  - `create_app()` - Pure function for application configuration

### 4. **Error Module** (`src/error.rs`)
- **Purpose**: Centralized error handling
- **Principles**: Custom error types, functional error handling
- **Key Functions**:
  - `map_io_error()` - Pure function for error conversion
  - `validate_file_path()` - Pure function for path validation

## Functional Programming Principles Applied

### 1. **Pure Functions**
- All functions are pure where possible (no side effects)
- Functions return the same output for the same input
- No global state mutations

### 2. **Immutability**
- Configuration is immutable after creation
- Data structures are cloned rather than mutated
- No mutable state in pure functions

### 3. **Composition**
- Functions are composed together to build complex behavior
- Each function has a single responsibility
- Functions can be easily tested in isolation

### 4. **Error Handling**
- Uses `Result<T, E>` for explicit error handling
- Custom error types for domain-specific errors
- Error propagation with `?` operator

### 5. **Type Safety**
- Strong typing throughout the codebase
- Custom types for domain concepts (VideoInfo, RangeRequest, etc.)
- Compile-time guarantees

## Usage

### Environment Variables
```bash
export VIDEO_PATH="videos/dg.webm"
export HOST="127.0.0.1"
export PORT="8080"
export CONTENT_TYPE="video/webm"
```

### Running the Server
```bash
cargo run
```

### Testing Individual Functions
```bash
# Test range parsing
cargo test parse_range_header

# Test video metadata
cargo test get_video_metadata

# Test configuration
cargo test config
```
