use chrono::Local;
use std::time::Duration;
use tokio::process::Command;

pub(crate) async fn grab_frame() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");

    let output_path = format!("frames/{}.png", timestamp);

    let status = tokio::time::timeout(
        Duration::from_secs(10), // 10 second timeout
        Command::new("ffmpeg")
            .args([
                "-y",
                "-loglevel", "error",
                "-i",
                "https://cctv.jogjaprov.go.id/cctv-proxy/cctv-public/ViewParangtritis.stream/playlist.m3u8",
                "-frames:v",
                "1",
                &output_path
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
    ).await??;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    Ok(output_path)
}