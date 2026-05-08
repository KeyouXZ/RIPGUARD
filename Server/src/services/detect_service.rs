use std::io::Cursor;
use axum::Json;
use base64::Engine;
use image::{load_from_memory, ImageFormat, Rgb};
use imageproc::{
    drawing::draw_hollow_rect_mut,
    rect::Rect
};
use log::info;
use base64::engine::general_purpose::STANDARD;
use image::codecs::jpeg::JpegEncoder;
use ort::session::Session;
use crate::{
    model::DetectionResponse,
    services::run_detection::run_detection
};

pub async fn detect(session: &mut Session, image_b64: String) -> Result<Json<DetectionResponse>, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    info!("Detect Called!");

    let cleaned = image_b64
        .replace("data:image/jpeg;base64,", "")
        .replace("data:image/png;base64,", "");
    let image_bytes = STANDARD.decode(cleaned)?;
    let img = load_from_memory(&image_bytes)?;

    let img = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
    let img = img.to_rgb8();

    let results = run_detection(session, &img)?;

    // Test purpose
    let mut out_img = img.clone();
    for det in &results {

        let x1 = det.bbox.x1.max(0.0) as u32;
        let y1 = det.bbox.y1.max(0.0) as u32;
        let x2 = det.bbox.x2.min(639.0) as u32;
        let y2 = det.bbox.y2.min(639.0) as u32;

        // Draw a red rectangle
        let rect = Rect::at(x1 as i32, y1 as i32).of_size(x2 - x1, y2 - y1);
        draw_hollow_rect_mut(&mut out_img, rect, Rgb([0, 0, 255]));
    }

    // Turn image into base64
    let mut buffer = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 75);

    encoder.encode_image(&out_img)?;

    let image_base64 = STANDARD.encode(buffer.into_inner());

    // Save the processed image
    #[cfg(debug_assertions)]
    out_img.save_with_format("processed.jpg", ImageFormat::Jpeg)?;

    let response = DetectionResponse {
        detections: results,
        image: image_base64
    };

    Ok(Json(response))
}