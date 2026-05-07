use std::process::exit;
use axum::Json;
use base64::Engine;
use image::{load_from_memory, Rgb};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use log::{error, info};
use onnxruntime::environment::Environment;
use onnxruntime::GraphOptimizationLevel;
use onnxruntime::ndarray::Array4;
use onnxruntime::tensor::OrtOwnedTensor;
use base64::engine::general_purpose::STANDARD;
use crate::{model};

pub async fn detect(image_b64: String) -> Result<Json<Vec<model::Detection>>, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
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

    let cleaned = image_b64
        .replace("data:image/jpeg;base64,", "")
        .replace("data:image/png;base64,", "");
    let image_bytes = STANDARD.decode(cleaned)?;
    let img = load_from_memory(&image_bytes)?;

    let img = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
    let img = img.to_rgb8();

    let mut input_array = Array4::<f32>::zeros((1, 3, 640, 640));

    for (x, y, pixel) in img.enumerate_pixels() {
        input_array[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
        input_array[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
        input_array[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
    }

    #[cfg(debug_assertions)]
    info!("Preprocessing done!");

    // =========================
    // 3. Run inference
    // =========================
    let input_tensor = vec![input_array.clone()];
    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor)?;

    #[cfg(debug_assertions)]
    info!("Inference done!");

    // =========================
    // 4. Post-process (basic)
    // =========================
    let output = &outputs[0];

    let shape = output.shape();

    let num_boxes = shape[1];

    let mut results = Vec::new();
    for i in 0..num_boxes {
        let conf = output[[0, i, 4]];

        #[cfg(debug_assertions)]
        info!("CONF = {}", conf);

        if conf > 0.01 {
            let x1 = output[[0, i, 0]];
            let y1 = output[[0, i, 1]];
            let x2 = output[[0, i, 2]];
            let y2 = output[[0, i, 3]];

            let class_id = output[[0, i, 5]];

            #[cfg(debug_assertions)]
            info!(
                "Detected → class: {}, conf: {}, x: {}, y: {}, w: {}, h: {}",
                class_id, conf, x1, y1, x2, y2
            );

            results.push(model::Detection {
                bbox: [x1, y1, x2, y2],
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

        let x1 = det.bbox[0].max(0.0) as u32;
        let y1 = det.bbox[1].max(0.0) as u32;
        let x2 = det.bbox[2].min(639.0) as u32;
        let y2 = det.bbox[3].min(639.0) as u32;

        // Draw a red rectangle
        let rect = Rect::at(x1 as i32, y1 as i32).of_size(x2 - x1, y2 - y1);
        draw_hollow_rect_mut(&mut out_img, rect, Rgb([0, 0, 255]));
    }

    // Save the processed image
    out_img.save("processed.jpg")?;

    Ok(Json(results))
}