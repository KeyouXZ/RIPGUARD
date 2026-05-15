use image::RgbImage;
use ndarray::Array4;
use ort::session::Session;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{Mutex, broadcast};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub time: String,       // iso8601
    pub interval: i32,      // seconds
    pub temperature: f64,   // °C
    pub windspeed: f64,     // km/h
    pub winddirection: i32, // °
    pub is_day: i32,        //
    pub weathercode: i32,   // wmo code
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub current_weather: CurrentWeather,
}

#[derive(Clone)]
pub struct AppState {
    pub session: Arc<Mutex<Session>>,
    pub req_client: reqwest::Client,
    pub config: Arc<crate::config::Config>,
    pub tx: broadcast::Sender<String>,
    pub input_buffer: Arc<Mutex<Array4<f32>>>,
    pub image_buffer: Arc<Mutex<RgbImage>>,
    pub cache: Arc<Mutex<VecDeque<Detection>>>,
}

#[derive(Debug, Serialize)]
pub struct DetectionResponse {
    pub detections: Vec<DetectionResult>,
    pub image: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorReport {
    pub message: String,
    pub platform: String,
    pub source: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum Payload {
    #[serde(rename = "init")]
    Init(VecDeque<Detection>),

    #[serde(rename = "detection")]
    Detection(Detection),
}
