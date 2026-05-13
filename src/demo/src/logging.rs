//! Demo logging: console + file output to `%LOCALAPPDATA%\Workspace\demo.log`.

use std::fs;
use std::path::PathBuf;

use tracing_subscriber::prelude::*;

fn demo_log_directory() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA")
            .ok()
            .map(|base| PathBuf::from(base).join("Workspace").join("logs"))
    }

    #[cfg(target_os = "linux")]
    {
        std::env::var("HOME").ok().map(|home| {
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("Workspace")
                .join("logs")
        })
    }

    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME").ok().map(|home| {
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Workspace")
                .join("logs")
        })
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

/// Initialize tracing with both console output and a log file.
///
/// Falls back to console-only if the log file cannot be created.
/// Uses `WORKSPACE_LOG` env var for level filtering (default: `info`).
pub fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_env("WORKSPACE_LOG")
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    match try_open_log_file() {
        Some((file, log_path)) => {
            let console_layer = tracing_subscriber::fmt::layer();
            let file_layer = tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(std::sync::Mutex::new(file));

            tracing_subscriber::registry()
                .with(env_filter)
                .with(console_layer)
                .with(file_layer)
                .init();

            tracing::info!(log_file = %log_path.display(), "demo logging initialized (console + file)");
        }
        None => {
            tracing_subscriber::fmt().with_env_filter(env_filter).init();

            tracing::info!("demo logging initialized (console only — log dir unavailable)");
        }
    }
}

fn try_open_log_file() -> Option<(fs::File, PathBuf)> {
    let log_dir = demo_log_directory()?;
    fs::create_dir_all(&log_dir).ok()?;
    let log_path = log_dir.join("demo.log");
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .ok()?;
    Some((file, log_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_log_directory_returns_valid_path() {
        if let Some(path) = demo_log_directory() {
            assert!(path.to_string_lossy().contains("Workspace"));
            assert!(path.to_string_lossy().contains("logs"));
        }
    }
}
