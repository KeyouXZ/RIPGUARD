// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use ripguard_model::{ApiResponse, Detection};
use std::time::Duration;

// controller.setCenter(GeoPoint(-8.0253993, 110.3287713))
const DEFAULT_LATITUDE: f64 = -8.0253993;
const DEFAULT_LONGITUDE: f64 = 110.3287713;

pub(crate) async fn enrich_weather(client: &reqwest::Client, detection: &mut Detection) {
    let (latitude, longitude) = (
        detection.latitude.unwrap_or(DEFAULT_LATITUDE),
        detection.longitude.unwrap_or(DEFAULT_LONGITUDE),
    );

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        latitude, longitude
    );

    let wind_speed = async {
        let res = tokio::time::timeout(Duration::from_secs(5), client.get(&url).send())
            .await
            .ok()?
            .unwrap();

        let data = res.json::<ApiResponse>().await.ok()?;

        Some(data.current_weather.windspeed as f32)
    }
    .await;

    detection.wind_speed = wind_speed;
}
