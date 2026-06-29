use crate::Detection;
use image::RgbImage;
use ndarray::Array4;
use ort::session::Session;
use ripguard_config::Config;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::{Mutex, broadcast};

#[derive(Clone)]
pub struct AppState {
    pub session: Arc<Mutex<Session>>,
    pub req_client: reqwest::Client,
    pub config: Arc<Config>,
    pub tx: broadcast::Sender<String>,
    pub input_buffer: Arc<Mutex<Array4<f32>>>,
    pub image_buffer: Arc<Mutex<RgbImage>>,
    pub cache: Arc<Mutex<VecDeque<Detection>>>,
    pub last_id: Arc<Mutex<u64>>,
}
