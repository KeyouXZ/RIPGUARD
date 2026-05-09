use std::{
    process::exit,
    sync::Arc
};
use chrono::Local;
use image::{ImageFormat, ImageReader};
use log::error;
use ort::session::Session;
use rand::RngExt;
use tokio::fs;
use tokio::sync::Mutex;
use crate::{
    model::{BoundingBox, Detection, DetectionResult},
    services::run_detection::run_detection
};
use crate::services::draw_rect::generate_output_img;

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
        }],
        latitude: rng.random_range(-8.1..-7.9),
        longitude: rng.random_range(110.2..110.5),
        wind_speed: None,
    }
}

pub(crate) async fn real_detection(session: &Arc<Mutex<Session>>, img_path: &str) -> Detection {
    let mut rng = rand::rng();

    let mut session = session.lock().await;

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

    let detections = match run_detection(&mut session, &img) {
        Ok(detections) => detections,
        Err(e) => {
            error!("Error: {}", e);
            exit(1);
        }
    };

    if !detections.is_empty() {
        let out_img = generate_output_img(&img, detections.clone());

        // Save the processed image
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        fs::create_dir_all("frames/detected").await.unwrap();

        let output_path = format!("frames/detected/{}.png", timestamp);
        out_img.save_with_format(output_path, ImageFormat::Jpeg).unwrap_or_else(|e| {
            error!("Error: {}", e);
        });
    }

    // TODO: Calculate latitude and longitude
    Detection {
        detections,
        latitude: rng.random_range(-8.1..-7.9),
        longitude: rng.random_range(110.2..110.5),
        wind_speed: None,
    }
}