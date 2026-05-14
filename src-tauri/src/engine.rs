use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use tokio::process::Command;

pub const SUPPORTED_IMAGE_EXTS: &[&str] = &[
    "jpg", "jpeg", "png", "tif", "tiff", "heic", "heif", "webp",
];

#[allow(dead_code)]
pub const SUPPORTED_VIDEO_EXTS: &[&str] = &["mp4", "mov", "m4v"];

pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            let lower = e.to_lowercase();
            SUPPORTED_IMAGE_EXTS.iter().any(|x| *x == lower)
        })
        .unwrap_or(false)
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub alt: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    // Address text from the lookup field. Stored alongside the file so the user can see
    // what address produced the current GPS pin when revisiting the image. Not written
    // to the image metadata itself (lat/lon are the truth).
    #[serde(default)]
    pub address: String,
}

impl MediaMetadata {
    pub fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.alt.is_empty()
            && self.latitude.is_none()
            && self.longitude.is_none()
    }
}

fn resolve_exiftool(app: &AppHandle) -> Result<PathBuf, AppError> {
    let path = app
        .path()
        .resolve("binaries/exiftool.exe", BaseDirectory::Resource)
        .map_err(|e| AppError::ExifTool(format!("Couldn't resolve bundled exiftool path: {e}")))?;
    if !path.is_file() {
        return Err(AppError::ExifTool(format!(
            "Bundled ExifTool missing at {}. Reinstall the app.",
            path.display()
        )));
    }
    Ok(path)
}

async fn run_exiftool(app: &AppHandle, args: Vec<String>) -> Result<String, AppError> {
    let exiftool_path = resolve_exiftool(app)?;

    let output = Command::new(&exiftool_path)
        .args(&args)
        .output()
        .await
        .map_err(|e| {
            AppError::ExifTool(format!(
                "Failed to launch ExifTool ({}): {e}",
                exiftool_path.display()
            ))
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    if !output.status.success() {
        return Err(AppError::ExifTool(format!(
            "exit {:?}: {}",
            output.status.code(),
            if stderr.trim().is_empty() {
                stdout.trim()
            } else {
                stderr.trim()
            }
        )));
    }
    Ok(stdout)
}

fn copy_preserving_mtime(src: &Path, dest: &Path) -> Result<(), AppError> {
    let dest_parent = dest
        .parent()
        .ok_or_else(|| AppError::InvalidPath(format!("Destination has no parent: {}", dest.display())))?;
    std::fs::create_dir_all(dest_parent)?;

    // Defense-in-depth: refuse to write a file onto itself. The caller in commands.rs
    // already remaps same-folder writes to a "-tagged" suffix, so this branch should
    // never fire in normal use — but if anything slips through, we don't want to
    // clobber an original.
    if let (Ok(src_canon), Ok(dest_canon)) =
        (dunce::canonicalize(src), dunce::canonicalize(dest_parent).map(|p| p.join(dest.file_name().unwrap_or_default())))
    {
        if src_canon == dest_canon {
            return Err(AppError::Other(format!(
                "Refusing to overwrite original file at {}",
                src.display()
            )));
        }
    }

    std::fs::copy(src, dest).map_err(|e| {
        AppError::Other(format!(
            "Failed to copy \"{}\" to \"{}\": {}",
            src.display(),
            dest.display(),
            e
        ))
    })?;
    let src_meta = std::fs::metadata(src)?;
    let mtime = filetime::FileTime::from_last_modification_time(&src_meta);
    let atime = filetime::FileTime::from_last_access_time(&src_meta);
    filetime::set_file_times(dest, atime, mtime)
        .map_err(|e| AppError::Other(format!("set_file_times: {e}")))?;
    Ok(())
}

fn build_metadata_args(metadata: &MediaMetadata) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "-overwrite_original".into(),
        "-codedcharacterset=utf8".into(),
        "-n".into(),
    ];

    if !metadata.alt.is_empty() {
        args.push(format!("-EXIF:ImageDescription={}", metadata.alt));
        args.push(format!("-XMP-dc:Description={}", metadata.alt));
        args.push(format!("-IPTC:Caption-Abstract={}", metadata.alt));
    }

    if !metadata.title.is_empty() {
        args.push(format!("-XMP-dc:Title={}", metadata.title));
        args.push(format!("-IPTC:ObjectName={}", metadata.title));
    }

    if let (Some(lat), Some(lon)) = (metadata.latitude, metadata.longitude) {
        let lat_ref = if lat >= 0.0 { "N" } else { "S" };
        let lon_ref = if lon >= 0.0 { "E" } else { "W" };
        args.push(format!("-EXIF:GPSLatitude={}", lat.abs()));
        args.push(format!("-EXIF:GPSLatitudeRef={}", lat_ref));
        args.push(format!("-EXIF:GPSLongitude={}", lon.abs()));
        args.push(format!("-EXIF:GPSLongitudeRef={}", lon_ref));
        args.push(format!("-XMP:GPSLatitude={lat}"));
        args.push(format!("-XMP:GPSLongitude={lon}"));
    }

    args
}

pub async fn write_metadata(
    app: &AppHandle,
    src: &Path,
    dest: &Path,
    metadata: &MediaMetadata,
) -> Result<PathBuf, AppError> {
    if !src.is_file() {
        return Err(AppError::InvalidPath(src.display().to_string()));
    }
    copy_preserving_mtime(src, dest)?;

    if metadata.is_empty() {
        return Ok(dest.to_path_buf());
    }

    let mut args = build_metadata_args(metadata);
    args.push(dest.display().to_string());

    run_exiftool(app, args).await?;
    Ok(dest.to_path_buf())
}
