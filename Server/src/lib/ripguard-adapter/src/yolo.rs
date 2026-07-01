// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

use ort::session::Session;
use std::process::exit;

pub fn create_yolov8_session() -> Session {
    Session::builder()
        .unwrap_or_else(|e| {
            log::error!("Failed to create session: {}", e);
            exit(1);
        })
        .commit_from_file("best.onnx")
        .unwrap_or_else(|e| {
            log::error!("Failed to load model: {}", e);
            exit(1);
        })
}
