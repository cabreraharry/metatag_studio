use crate::engine::{self, MediaMetadata, is_supported_image};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
}

impl FileEntry {
    fn from_path(p: &std::path::Path) -> Self {
        Self {
            path: p.display().to_string(),
            name: p
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
        }
    }
}

#[tauri::command]
pub async fn add_paths(paths: Vec<String>) -> Result<Vec<FileEntry>, AppError> {
    let mut out = Vec::new();
    for raw in paths {
        let p = PathBuf::from(&raw);
        let canon = dunce::canonicalize(&p).unwrap_or(p);
        if canon.is_dir() {
            let entries = std::fs::read_dir(&canon)?;
            for entry in entries {
                let entry = entry?;
                let ep = entry.path();
                if ep.is_file() && is_supported_image(&ep) {
                    out.push(FileEntry::from_path(&ep));
                }
            }
        } else if canon.is_file() && is_supported_image(&canon) {
            out.push(FileEntry::from_path(&canon));
        }
    }
    Ok(out)
}

#[derive(Debug, Deserialize)]
pub struct ProcessOneArgs {
    pub src: String,
    pub output_dir: String,
    pub metadata: MediaMetadata,
}

#[derive(Debug, Serialize)]
pub struct ProcessResult {
    pub dest: String,
}

#[tauri::command]
pub async fn process_one(
    app: AppHandle,
    args: ProcessOneArgs,
) -> Result<ProcessResult, AppError> {
    let src = PathBuf::from(&args.src);
    let file_name = src
        .file_name()
        .ok_or_else(|| AppError::InvalidPath(args.src.clone()))?;
    let dest = PathBuf::from(&args.output_dir).join(file_name);

    let written = engine::write_metadata(&app, &src, &dest, &args.metadata).await?;
    Ok(ProcessResult {
        dest: written.display().to_string(),
    })
}

#[derive(Debug, Serialize)]
pub struct GeocodeResult {
    pub latitude: f64,
    pub longitude: f64,
    pub display_name: String,
}

#[tauri::command]
pub async fn geocode(address: String) -> Result<GeocodeResult, AppError> {
    let trimmed = address.trim();
    if trimmed.is_empty() {
        return Err(AppError::Other("Address is empty".into()));
    }

    let client = reqwest::Client::builder()
        .user_agent("MetaTagStudio/0.1 (https://github.com/USPrimeRealty/metatag-studio)")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| AppError::Other(format!("http client init: {e}")))?;

    let resp = client
        .get("https://nominatim.openstreetmap.org/search")
        .query(&[
            ("q", trimmed),
            ("format", "json"),
            ("limit", "1"),
        ])
        .send()
        .await
        .map_err(|e| AppError::Other(format!("geocode request failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::Other(format!(
            "geocode returned HTTP {}",
            resp.status()
        )));
    }

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::Other(format!("geocode JSON parse: {e}")))?;

    let first = body
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| AppError::Other(format!("No results for: {trimmed}")))?;

    let lat: f64 = first
        .get("lat")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| AppError::Other("Missing lat".into()))?;
    let lon: f64 = first
        .get("lon")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| AppError::Other("Missing lon".into()))?;
    let display_name = first
        .get("display_name")
        .and_then(|v| v.as_str())
        .unwrap_or(trimmed)
        .to_string();

    Ok(GeocodeResult {
        latitude: lat,
        longitude: lon,
        display_name,
    })
}
