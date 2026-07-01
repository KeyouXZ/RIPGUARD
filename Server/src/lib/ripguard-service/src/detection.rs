// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{generate_output_img, run_detection};
use chrono::Local;
use image::{ImageFormat, ImageReader};
use log::error;
use ripguard_model::{AppState, Detection};
use std::process::exit;

#[cfg(feature = "fake_detections")]
use rand::RngExt;
#[cfg(feature = "fake_detections")]
use ripguard_model::{BoundingBox, DetectionResult};

#[cfg(feature = "fake_detections")]
pub(crate) fn fake_detections() -> Detection {
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
        latitude: None,
        longitude: None,
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

    let mut last_id = app_state.last_id.lock().await;
    let id = *last_id;
    *last_id += 1;

    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    // TODO: Calculate latitude, longitude
    Detection {
        id,
        detections,
        latitude: None,
        longitude: None,
        wind_speed: None,
        created_at,
        image_path: output_path,
        ttl: 60 * 60,
    }
}
