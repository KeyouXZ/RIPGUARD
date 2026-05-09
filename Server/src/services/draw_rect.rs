use ab_glyph::{FontRef, PxScale};
use image::RgbImage;
use crate::model::DetectionResult;
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

pub fn generate_output_img(img: &RgbImage, results: Vec<DetectionResult>)  -> RgbImage {
    let mut out_img = img.clone();

    let font_data = include_bytes!("../../assets/fonts/Arial.ttf");
    let font = FontRef::try_from_slice(font_data as &[u8]).unwrap();

    let scale = PxScale::from(20.0);
    let text_offset = 24;

    for det in &results {
        let x1 = det.bbox.x1.max(0.0) as i32;
        let y1 = det.bbox.y1.max(0.0) as i32;
        let x2 = det.bbox.x2.min(639.0) as i32;
        let y2 = det.bbox.y2.min(639.0) as i32;

        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);

        // draw rectangle
        draw_hollow_rect_mut(&mut out_img, rect, image::Rgb([255, 0, 0]));

        let label = format!("RIP CURRENT {:.2}%", det.confidence * 100.0);

        let text_x = x1;
        let text_y = (y1 - text_offset).max(0);

        let label_width = (label.len() as i32 + 1) * 10;
        let label_height = 22;

        let label_bg = Rect::at(text_x, text_y)
            .of_size(label_width as u32, label_height as u32);

        draw_filled_rect_mut(
            &mut out_img,
            label_bg,
            image::Rgb([255, 0, 0]),
        );

        draw_text_mut(
            &mut out_img,
            image::Rgb([255, 255, 255]),
            text_x,
            text_y,
            scale,
            &font,
            &label,
        );
    };

    out_img
}