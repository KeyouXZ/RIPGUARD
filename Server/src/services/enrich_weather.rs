use std::time::Duration;
use crate::model::{ApiResponse, Detection};

pub(crate) async fn enrich_weather(client: &reqwest::Client, detection: &mut Detection) {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        detection.latitude, detection.longitude
    );

    let wind_speed = async {
        let res = tokio::time::timeout(
            Duration::from_secs(5),
            client.get(&url).send()
        ).await.ok()?.unwrap();

        let data = res.json::<ApiResponse>().await.ok()?;

        Some(data.current_weather.windspeed as f32)
    }
    .await;

    detection.wind_speed = wind_speed;
}
