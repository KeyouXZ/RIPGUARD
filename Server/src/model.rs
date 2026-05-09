use std::sync::Arc;
use ort::session::Session;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detection {
    pub detections: Vec<DetectionResult>,

    pub latitude: f64,
    pub longitude: f64,

    pub wind_speed: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub time: String, // iso8601
    pub interval: i32, // seconds
    pub temperature: f64, // °C
    pub windspeed: f64, // km/h
    pub winddirection: i32, // °
    pub is_day: i32, //
    pub weathercode: i32, // wmo code
}

#[derive(Deserialize)]
pub struct ApiResponse {
    pub current_weather: CurrentWeather,
}

#[derive(Clone)]
pub struct AppState {
    pub session: Arc<Mutex<Session>>,
    pub req_client: reqwest::Client,
    pub config: crate::config::Config,
    pub tx: broadcast::Sender<String>,
}

#[derive(Serialize)]
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