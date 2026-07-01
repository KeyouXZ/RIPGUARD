// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

use image::RgbImage;
#[cfg(debug_assertions)]
use log::info;
use ndarray::Array4;
use ort::{inputs, session::Session, value::TensorRef};
use ripguard_model::{BoundingBox, DetectionResult};

pub fn run_detection(
    session: &mut Session,
    img: &RgbImage,
    input_buffer: &mut Array4<f32>,
) -> Result<Vec<DetectionResult>, Box<dyn std::error::Error>> {
    // =========================
    // Preprocess
    // =========================
    // Reuse pre-allocated buffer instead of allocating new one each time
    input_buffer.fill(0.0);

    // Bulk operation: access raw pixel data for better performance
    let width = img.width() as usize;
    let height = img.height() as usize;
    let raw_pixels = img.as_raw();

    // Process all pixels in bulk - much faster than enumerate_pixels()
    for (i, chunk) in raw_pixels.chunks_exact(3).enumerate() {
        let y = i / width;
        let x = i % width;

        if y < height && x < width {
            input_buffer[[0, 0, y, x]] = chunk[0] as f32 / 255.0;
            input_buffer[[0, 1, y, x]] = chunk[1] as f32 / 255.0;
            input_buffer[[0, 2, y, x]] = chunk[2] as f32 / 255.0;
        }
    }

    #[cfg(debug_assertions)]
    info!("Preprocessing done!");

    // =========================
    // Inference
    // =========================

    let input_tensor = TensorRef::from_array_view(input_buffer)?;

    let outputs = session.run(inputs![input_tensor])?;

    #[cfg(debug_assertions)]
    info!("Inference done!");

    // =========================
    // Postprocess
    // =========================

    let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;

    let num_boxes = shape[1] as usize;
    let num_attrs = shape[2] as usize;

    let idx = |box_idx: usize, attr_idx: usize| -> usize { box_idx * num_attrs + attr_idx };

    let mut results = Vec::new();

    for i in 0..num_boxes {
        let conf = data[idx(i, 4)];
        let class_id = data[idx(i, 5)];

        if conf <= 0.5 || class_id != 1.0 {
            continue;
        }

        let x1 = data[idx(i, 0)];
        let y1 = data[idx(i, 1)];
        let x2 = data[idx(i, 2)];
        let y2 = data[idx(i, 3)];

        #[cfg(debug_assertions)]
        info!("Detected → class: {}, conf: {}", class_id, conf);

        results.push(DetectionResult {
            bbox: BoundingBox { x1, y1, x2, y2 },
            confidence: conf,
        });
    }

    Ok(results)
}
