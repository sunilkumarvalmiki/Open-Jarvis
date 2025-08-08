use std::{fs, path::Path};
use tauri::Manager;

#[tauri::command]
fn open_browser(app: tauri::AppHandle, url: String) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        use std::time::Duration;
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = tauri::api::shell::open(&app.shell_scope(), url, None);
    });
    Ok(())
}

#[tauri::command]
fn empty_recycle_bin() -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        use std::time::Duration;
        tokio::time::sleep(Duration::from_secs(2)).await;
        empty_bin_impl().ok();
    });
    Ok(())
}

#[tauri::command]
fn organize_files() -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        use std::time::Duration;
        tokio::time::sleep(Duration::from_secs(2)).await;
        if let Some(home) = dirs_next::download_dir() {
            organize_impl(&home).ok();
        }
    });
    Ok(())
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
            std::process::Command::new("rm")
                .arg("-rf")
                .arg(trash.join("*"))
                .status()
                .map_err(|e| e.to_string())?;
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

fn organize_impl(dir: &Path) -> Result<(), String> {
    let docs = dir.join("Documents");
    let music = dir.join("Music");
    let pics = dir.join("Pictures");
    fs::create_dir_all(&docs).map_err(|e| e.to_string())?;
    fs::create_dir_all(&music).map_err(|e| e.to_string())?;
    fs::create_dir_all(&pics).map_err(|e| e.to_string())?;

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let dest = match ext.to_lowercase().as_str() {
                    "pdf" | "doc" | "docx" | "txt" => Some(docs.join(entry.file_name())),
                    "mp3" | "wav" | "flac" | "aac" => Some(music.join(entry.file_name())),
                    "jpg" | "jpeg" | "png" | "gif" => Some(pics.join(entry.file_name())),
                    _ => None,
                };
                if let Some(d) = dest {
                    let _ = fs::rename(&path, d);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_browser, empty_recycle_bin, organize_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
