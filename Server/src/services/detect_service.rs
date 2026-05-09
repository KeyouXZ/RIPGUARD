use std::io::Cursor;
use axum::body::Bytes;
use base64::Engine;
use image::load_from_memory;
use image::codecs::jpeg::JpegEncoder;
#[cfg(debug_assertions)]
use image::ImageFormat;
#[cfg(debug_assertions)]
use log::info;
use base64::engine::general_purpose::STANDARD;
use ort::session::Session;
use crate::{
    model::DetectionResponse,
    services::run_detection::run_detection
};
use crate::services::draw_rect::generate_output_img;

pub async fn detect(session: &mut Session, image_bytes: Bytes) -> Result<DetectionResponse, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    info!("Detect Called!");

    let img = load_from_memory(&image_bytes)?;

    let img = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
    let img = img.to_rgb8();

    let results = run_detection(session, &img)?;

    let out_img = generate_output_img(&img, results.clone());

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

    Ok(response)
}