use std::{
    process::exit,
    sync::Arc
};
use image::ImageReader;
use log::error;
use ort::session::Session;
use rand::RngExt;
use tokio::sync::Mutex;
use crate::{
    model::{BoundingBox, Detection, DetectionResult},
    services::run_detection::run_detection
};

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

    // TODO: Calculate latitude and longitude
    Detection {
        detections,
        latitude: rng.random_range(-8.1..-7.9),
        longitude: rng.random_range(110.2..110.5),
        wind_speed: None,
    }
}