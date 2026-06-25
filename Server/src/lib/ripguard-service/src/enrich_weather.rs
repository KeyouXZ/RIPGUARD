use ripguard_model::{ApiResponse, Detection};
use std::time::Duration;

pub(crate) async fn enrich_weather(client: &reqwest::Client, detection: &mut Detection) {
    let Some(first_detection) = detection.detections.first() else {
        return;
    };

    let (Some(latitude), Some(longitude)) = (first_detection.latitude, first_detection.longitude)
    else {
        return;
    };

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
