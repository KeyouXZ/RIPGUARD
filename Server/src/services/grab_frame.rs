use chrono::Local;
use tokio::{
    fs,
    process::Command
};

pub(crate) async fn grab_frame() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    fs::create_dir_all("frames").await?;

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let output_path = format!("frames/{}.png", timestamp);

    let status = Command::new("ffmpeg")
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
        .await?;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    Ok(output_path)
}