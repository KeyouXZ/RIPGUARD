// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

use log::{error, info};
use std::time::SystemTime;
use tokio::time::{self, Duration};

async fn directory_size(path: &str) -> std::io::Result<u64> {
    let mut size = 0u64;
    let mut entries = tokio::fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let meta = entry.metadata().await?;
        if meta.is_file() {
            size += meta.len();
        }
    }
    Ok(size)
}

async fn get_oldest_files(path: &str) -> std::io::Result<Vec<(std::path::PathBuf, u64)>> {
    let mut files = Vec::new();
    let mut entries = tokio::fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let meta = entry.metadata().await?;
        if meta.is_file() {
            let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let size = meta.len();
            files.push((entry.path(), modified, size));
        }
    }
    // Oldest first
    files.sort_by_key(|a| a.1);
    Ok(files.into_iter().map(|(p, _, s)| (p, s)).collect())
}

pub async fn start_file_deletor(interval: Duration, path: &str, threshold: u64) {
    let mut interval = time::interval(interval);
    loop {
        interval.tick().await;

        let size = match directory_size(path).await {
            Ok(s) => s,
            Err(e) => {
                error!("Cannot read directory size: {}", e);
                continue;
            }
        };

        if size > threshold {
            let files = match get_oldest_files(path).await {
                Ok(f) => f,
                Err(e) => {
                    error!("Cannot read directory: {}", e);
                    continue;
                }
            };

            let mut deleted = 0;
            let mut freed: u64 = 0;

            // Delete the 10 oldest files
            for (file_path, file_size) in files.iter().take(10) {
                match tokio::fs::remove_file(file_path).await {
                    Ok(_) => {
                        deleted += 1;
                        freed += file_size;
                    }
                    Err(e) => {
                        error!("Failed to delete {}: {}", file_path.display(), e);
                    }
                }
            }

            info!(
                "Threshold exceeded ({} > {}). Deleted {} oldest files, freed {} bytes.",
                size, threshold, deleted, freed
            );
        }
    }
}
