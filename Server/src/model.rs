use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detection {
    pub bbox: [f32; 4],
    pub confidence: f32,

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