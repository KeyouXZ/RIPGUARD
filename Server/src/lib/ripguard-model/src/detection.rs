// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub id: u64,
    pub detections: Vec<DetectionResult>,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub wind_speed: Option<f32>,

    /// In milliseconds
    pub created_at: u64,
    pub image_path: Option<String>,

    /// In seconds
    pub ttl: u64,
}

impl Detection {
    pub fn is_expired(&self) -> bool {
        if self.ttl == 0 {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let expires_at = self
            .created_at
            .saturating_add(self.ttl.saturating_mul(1000));

        now >= expires_at
    }
}
