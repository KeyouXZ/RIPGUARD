use std::process::exit;
use axum::Json;
use image::{ImageReader, Rgb};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use log::{error, info};
use onnxruntime::environment::Environment;
use onnxruntime::GraphOptimizationLevel;
use onnxruntime::ndarray::Array4;
use onnxruntime::tensor::OrtOwnedTensor;
use crate::{model};

pub async fn detect() -> Result<Json<Vec<model::Detection>>, Box<dyn std::error::Error>> {
    info!("Detect Called!");

    let environment = Environment::builder()
        .with_name("yolo")
        .build()
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            exit(1);
        });

    let session = environment.new_session_builder().unwrap_or_else(|e| {
        error!("Error: {}", e);
        exit(1);
    });

    let mut session = session
        .with_optimization_level(GraphOptimizationLevel::Basic)
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            exit(1);
        })
        .with_number_threads(1)
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            exit(1);
        })
        .with_model_from_file("best.onnx")
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            exit(1);
        });

    let img = ImageReader::open("test.jpeg")
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

    let mut input_array = Array4::<f32>::zeros((1, 3, 640, 640));

    for (x, y, pixel) in img.enumerate_pixels() {
        input_array[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
        input_array[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
        input_array[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
    }

    info!("Preprocessing done!");

    // =========================
    // 3. Run inference
    // =========================
    let input_tensor = vec![input_array.clone()];
    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor)?;

    info!("Inference done!");

    // =========================
    // 4. Post-process (basic)
    // =========================
    let output = &outputs[0];
    let shape = output.shape();

    info!("Output shape: {:?}", shape);

    let num_boxes = shape[2];

    let mut results = Vec::new();
    for i in 0..num_boxes {
        let conf = 1.0 / (1.0 + (-output[[0, 4, i]]).exp());
        if conf > 0.5 {
            let x = output[[0, 0, i]];
            let y = output[[0, 1, i]];
            let w = output[[0, 2, i]];
            let h = output[[0, 3, i]];

            info!(
                "Detected → x: {}, y: {}, w: {}, h: {}, conf: {}",
                x, y, w, h, conf
            );

            results.push(model::Detection {
                bbox: [x, y, w, h],
                confidence: conf,

                latitude: 0.0,
                longitude: 0.0,

                wind_speed: Some(0.0),
            })
        }
    }

    // Test purpose

    let mut out_img = img.clone();
    for det in &results {
        // YOLO outputs (x, y, w, h)
        let cx = det.bbox[0];
        let cy = det.bbox[1];
        let w = det.bbox[2];
        let h = det.bbox[3];

        // Convert center coordinates to top-left
        let x1 = (cx - w / 2.0).max(0.0) as u32;
        let y1 = (cy - h / 2.0).max(0.0) as u32;
        let x2 = (cx + w / 2.0).min(639.0) as u32;
        let y2 = (cy + h / 2.0).min(639.0) as u32;

        // Draw a red rectangle
        let rect = Rect::at(x1 as i32, y1 as i32).of_size(x2 - x1, y2 - y1);
        draw_hollow_rect_mut(&mut out_img, rect, Rgb([0, 0, 255]));
    }

    // Save the processed image
    out_img.save("processed.jpg")?;

    Ok(Json(results))
}