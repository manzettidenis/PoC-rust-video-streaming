use actix_web::{web, App, HttpServer, Responder,HttpRequest, HttpResponse, Result };
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom };

async fn stream_video(req: HttpRequest) -> Result<impl Responder> {
    let file_path: &'static str = "videos/dg.webm";
    let file: File  = File::open(file_path)?;
    let metadata: std::fs::Metadata = file.metadata()?;
    let total_size: u64 = metadata.len();

    let range_header = req.headers().get("Range").map(|h| h.to_str().ok()).flatten();
    let (start, end) = if let Some(range) = range_header {
       let parts: Vec<&str> = range.trim_start_matches("bytes=").split('-').collect();
        let start = parts[0].parse::<u64>().unwrap_or(0);
        let end = parts.get(1).and_then(|&s| s.parse::<u64>().ok()).unwrap_or(total_size - 1);
        (start, end)
    } else {
        (0, total_size - 1)
    };

    let mut reader: BufReader<File> = BufReader::new(file);
    reader.seek(SeekFrom::Start(start))?;
    let mut buffer: Vec<u8> = Vec::new();
    let length: usize = (end - start + 1) as usize;
    reader.take(length as u64).read_to_end(&mut buffer)?;

    Ok(HttpResponse::PartialContent()
        .content_type("video/webm")
        .append_header(("Content-Range", format!("bytes {}-{}/{}", start, end, total_size)))
        .body(buffer)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/stream", web::get().to(stream_video))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
