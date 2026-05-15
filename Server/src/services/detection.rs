use crate::services::draw_rect::generate_output_img;
use crate::{
    model::{AppState, BoundingBox, Detection, DetectionResult},
    services::run_detection::run_detection,
};
use chrono::Local;
use image::{ImageFormat, ImageReader};
use log::error;
use rand::RngExt;
use std::process::exit;

pub(crate) fn _fake_detections() -> Detection {
    let mut rng = rand::rng();

    Detection {
        detections: vec![DetectionResult {
            bbox: BoundingBox {
                x1: rng.random_range(0.0..0.5),
                y1: rng.random_range(0.0..0.5),
                x2: rng.random_range(0.5..1.0),
                y2: rng.random_range(0.5..1.0),
            },
            confidence: rng.random_range(0.5..1.0),
            latitude: None,
            longitude: None,
        }],
        wind_speed: None,
        created_at: std::time::SystemTime::now(),
        image_path: None,
    }
}

pub(crate) async fn real_detection(app_state: &AppState, img_path: &str) -> Detection {
    let img = ImageReader::open(img_path)
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            exit(1);
        })
        .decode()
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            exit(1);
        });

    let img = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
    let img = img.to_rgb8();

    let res = {
        let mut session = app_state.session.lock().await;
        let mut input_buffer = app_state.input_buffer.lock().await;

        run_detection(&mut session, &img, &mut input_buffer)
    };

    let detections = match res {
        Ok(detections) => detections,
        Err(e) => {
            error!("Error: {}", e);
            exit(1);
        }
    };

    let mut output_path = None;

    if !detections.is_empty() {
        // Create a mutable copy for drawing
        let mut out_img = img.clone();
        generate_output_img(&mut out_img, &detections);

        // Save the processed image
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        output_path = Some(format!("frames/detected/{}.png", timestamp));
        out_img
            .save_with_format(
                output_path.as_deref().unwrap_or_default(),
                ImageFormat::Jpeg,
            )
            .unwrap_or_else(|e| {
                error!("Error: {}", e);
            });
    }

    Detection {
        detections,
        wind_speed: None,
        created_at: std::time::SystemTime::now(),
        image_path: output_path,
    }
}
