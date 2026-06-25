use axum::body::Bytes;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
#[cfg(debug_assertions)]
use image::ImageFormat;
use image::codecs::jpeg::JpegEncoder;
use image::{GenericImage, load_from_memory};
#[cfg(debug_assertions)]
use log::info;
use ripguard_model::{AppState, DetectionResponse};
use std::io::Cursor;

use crate::{generate_output_img, run_detection};

pub async fn detect(
    app_state: &AppState,
    image_bytes: Bytes,
) -> Result<DetectionResponse, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    info!("Detect Called!");

    let img = load_from_memory(&image_bytes)?;

    let img = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
    let img = img.to_rgb8();

    // Use reusable buffer to avoid repeated allocations
    let results = {
        let mut session = app_state.session.lock().await;
        let mut input_buffer = app_state.input_buffer.lock().await;
        run_detection(&mut session, &img, &mut input_buffer)?
    };

    // Use reusable image buffer to avoid clone
    let response = {
        let mut image_buffer = app_state.image_buffer.lock().await;
        image_buffer.copy_from(&img, 0, 0)?;
        generate_output_img(&mut image_buffer, &results);

        // Encode while holding lock to prevent extra copies
        let mut buffer = Cursor::new(Vec::new());
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 75);
        encoder.encode_image(&*image_buffer)?;

        let image_base64 = STANDARD.encode(buffer.into_inner());

        // Save the processed image
        #[cfg(debug_assertions)]
        image_buffer.save_with_format("processed.jpg", ImageFormat::Jpeg)?;

        DetectionResponse {
            detections: results,
            image: image_base64,
        }
    };

    Ok(response)
}
