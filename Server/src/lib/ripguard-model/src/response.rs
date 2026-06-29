use std::collections::VecDeque;

use crate::{Detection, DetectionResult};
use serde::{Deserialize, Serialize};

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

    #[serde(rename = "new")]
    Detection(Detection),

    #[serde(rename = "del")]
    Delete(u64),
}
