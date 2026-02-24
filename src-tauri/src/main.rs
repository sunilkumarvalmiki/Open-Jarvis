use std::{fs, path::Path};

mod error;
mod mcp;

#[tauri::command]
async fn open_browser(app: tauri::AppHandle, url: String) -> Result<String, String> {
    tauri::api::shell::open(&app.shell_scope(), url.clone(), None).map_err(|e| e.to_string())?;
    Ok(format!("Opened {} successfully", url))
}

#[tauri::command]
async fn empty_recycle_bin() -> Result<String, String> {
    empty_bin_impl()?;
    Ok("Recycle bin emptied successfully".to_string())
}

#[tauri::command]
async fn organize_files() -> Result<String, String> {
    if let Some(home) = dirs_next::download_dir() {
        organize_impl(&home)
    } else {
        Err("Could not find downloads directory".to_string())
    }
}

fn empty_bin_impl() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "Clear-RecycleBin -Force"])
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("Failed to empty recycle bin".to_string());
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_next::home_dir() {
            let trash = home.join(".Trash");
            if trash.exists() {
                for entry in fs::read_dir(&trash).map_err(|e| e.to_string())? {
                    let entry = entry.map_err(|e| e.to_string())?;
                    let path = entry.path();
                    if path.is_dir() {
                        fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
                    } else {
                        fs::remove_file(&path).map_err(|e| e.to_string())?;
                    }
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = dirs_next::home_dir() {
            let trash = home.join(".local/share/Trash/files");
            let _ = fs::remove_dir_all(&trash);
            let _ = fs::create_dir_all(&trash);
        }
    }
    Ok(())
}

fn organize_impl(dir: &Path) -> Result<String, String> {
    let docs = dir.join("Documents");
    let music = dir.join("Music");
    let pics = dir.join("Pictures");
    let videos = dir.join("Videos");
    let archives = dir.join("Archives");
    let code = dir.join("Code");

    for d in [&docs, &music, &pics, &videos, &archives, &code] {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }

    let mut moved_count = 0;

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let dest = match ext.to_lowercase().as_str() {
                    "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "xls" | "xlsx" | "ppt"
                    | "pptx" | "csv" => Some(&docs),
                    "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => Some(&music),
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff"
                    | "heic" => Some(&pics),
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => {
                        Some(&videos)
                    }
                    "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => Some(&archives),
                    "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "go" | "java" | "c" | "cpp"
                    | "h" | "hpp" | "rb" | "php" | "swift" | "kt" | "scala" => Some(&code),
                    _ => None,
                };
                if let Some(dest_dir) = dest {
                    let mut dest_path = dest_dir.join(entry.file_name());
                    if dest_path.exists() {
                        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        let mut counter = 1;
                        loop {
                            let new_name = if extension.is_empty() {
                                format!("{}_{}", file_stem, counter)
                            } else {
                                format!("{}_{}.{}", file_stem, counter, extension)
                            };
                            dest_path = dest_dir.join(new_name);
                            if !dest_path.exists() {
                                break;
                            }
                            counter += 1;
                        }
                    }
                    if fs::rename(&path, dest_path).is_ok() {
                        moved_count += 1;
                    }
                }
            }
        }
    }
    Ok(format!("Organized {} files successfully", moved_count))
}

fn main() {
    env_logger::init();
    log::info!("Starting Jarvis application");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_browser,
            empty_recycle_bin,
            organize_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
