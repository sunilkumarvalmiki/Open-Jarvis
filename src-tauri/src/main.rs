use std::{fs, path::Path, time::Duration};
use tauri::Manager;

// MCP integration module (stub implementation)
mod mcp;

// Error types module
mod error;

#[tauri::command]
async fn open_browser(app: tauri::AppHandle, url: String) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    let url_clone = url.clone();
    tauri::api::shell::open(&app.shell_scope(), url, None).map_err(|e| e.to_string())?;
    Ok(format!("Opened {} successfully", url_clone))
}

#[tauri::command]
async fn empty_recycle_bin() -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    empty_bin_impl()?;
    Ok("Recycle bin emptied successfully".to_string())
}

#[tauri::command]
async fn organize_files() -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    if let Some(home) = dirs_next::download_dir() {
        organize_impl(&home)
    } else {
        Err("Could not find downloads directory".to_string())
    }
}

fn empty_bin_impl() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "Clear-RecycleBin -Force"])
            .status()
            .map_err(|e| e.to_string())?;
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

    // Create all directories
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
                    // Documents
                    "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "xls" | "xlsx" | "ppt"
                    | "pptx" | "csv" => Some(&docs),
                    // Music
                    "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => Some(&music),
                    // Pictures
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff"
                    | "heic" => Some(&pics),
                    // Videos
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => Some(&videos),
                    // Archives
                    "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => Some(&archives),
                    // Code
                    "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "go" | "java" | "c" | "cpp"
                    | "h" | "hpp" | "rb" | "php" | "swift" | "kt" | "scala" => Some(&code),
                    _ => None,
                };
                if let Some(dest_dir) = dest {
                    let dest_path = dest_dir.join(entry.file_name());
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
