use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoundingBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectionResult {
    pub bbox: BoundingBox,
    pub confidence: f32,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detection {
    pub detections: Vec<DetectionResult>,

    pub wind_speed: Option<f32>,

    pub created_at: SystemTime,
    pub image_path: Option<String>,
}

impl Detection {
    pub fn is_expired(&self, ttl: u64) -> bool {
        self.created_at
            .elapsed()
            .map(|e| e > Duration::from_millis(ttl))
            .unwrap_or(true)
    }
}
