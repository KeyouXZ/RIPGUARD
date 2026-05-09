use image::RgbImage;
#[cfg(debug_assertions)]
use log::info;
use ndarray::Array4;
use ort::inputs;
use ort::session::Session;
use ort::value::TensorRef;
use crate::model;

pub fn run_detection(
    session: &mut Session,
    img: &RgbImage,
) -> Result<Vec<model::DetectionResult>, Box<dyn std::error::Error>> {

    // =========================
    // Preprocess
    // =========================
    let mut input_array = Array4::<f32>::zeros((1, 3, 640, 640));

    for (x, y, pixel) in img.enumerate_pixels() {

        input_array[[0, 0, y as usize, x as usize]] =
            pixel[0] as f32 / 255.0;

        input_array[[0, 1, y as usize, x as usize]] =
            pixel[1] as f32 / 255.0;

        input_array[[0, 2, y as usize, x as usize]] =
            pixel[2] as f32 / 255.0;
    }

    #[cfg(debug_assertions)]
    info!("Preprocessing done!");

    // =========================
    // Inference
    // =========================

    let input_tensor = TensorRef::from_array_view(&input_array)?;

    let outputs = session.run(inputs![input_tensor])?;

    #[cfg(debug_assertions)]
    info!("Inference done!");

    // =========================
    // Postprocess
    // =========================

    let (shape, data) = outputs[0]
        .try_extract_tensor::<f32>()?;

    let num_boxes = shape[1] as usize;
    let num_attrs = shape[2] as usize;

    let idx = |box_idx: usize, attr_idx: usize| -> usize {
        box_idx * num_attrs + attr_idx
    };

    let mut results = Vec::new();

    for i in 0..num_boxes {

        let conf = data[idx(i, 4)];

        if conf <= 0.5 {
            continue;
        }

        let x1 = data[idx(i, 0)];
        let y1 = data[idx(i, 1)];
        let x2 = data[idx(i, 2)];
        let y2 = data[idx(i, 3)];

        #[cfg(debug_assertions)]
        let class_id = data[idx(i, 5)];

        #[cfg(debug_assertions)]
        info!(
            "Detected → class: {}, conf: {}",
            class_id,
            conf
        );

        results.push(model::DetectionResult {
            bbox: model::BoundingBox {
                x1,
                y1,
                x2,
                y2,
            },
            confidence: conf,
        });
    }

    Ok(results)
}