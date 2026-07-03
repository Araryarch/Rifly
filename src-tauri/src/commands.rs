use std::sync::{Arc, Mutex};
use crate::domain::track::Track;
use crate::infrastructure::metadata;
use crate::engine::player::Player;
use crate::services::discord::DiscordService;

pub struct AppState {
    pub player: Arc<Mutex<Player>>,
    pub _discord: Mutex<DiscordService>,
}

// --- Playback ---

#[tauri::command]
pub fn play_track(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    state.player.lock().map_err(|e| e.to_string())?.play(&path)
}

#[tauri::command]
pub fn pause_track(state: tauri::State<AppState>) -> Result<(), String> {
    state.player.lock().map_err(|e| e.to_string())?.pause();
    Ok(())
}

#[tauri::command]
pub fn resume_track(state: tauri::State<AppState>) -> Result<(), String> {
    state.player.lock().map_err(|e| e.to_string())?.resume();
    Ok(())
}

#[tauri::command]
pub fn stop_track(state: tauri::State<AppState>) -> Result<(), String> {
    state.player.lock().map_err(|e| e.to_string())?.stop();
    Ok(())
}

// --- Folder scanning ---

#[tauri::command]
pub fn scan_folder(dir: String) -> Result<Vec<Track>, String> {
    crate::infrastructure::scanner::scan_directory(&dir)
}

#[tauri::command]
pub fn read_track_metadata(path: String) -> Result<Track, String> {
    let p = std::path::Path::new(&path);
    metadata::read_metadata(p)
}

// --- Cover Art ---

#[tauri::command]
pub fn get_cover_art(path: String) -> Result<Option<String>, String> {
    let p = std::path::Path::new(&path);
    match metadata::read_cover_art(p) {
        Ok(b64) => Ok(Some(b64)),
        Err(_) => Ok(None),
    }
}

// --- Settings (simple JSON file) ---

fn settings_path() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap_or_default();
    p.pop();
    p.push("rifly_settings.json");
    p
}

fn read_settings() -> std::collections::HashMap<String, String> {
    let path = settings_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn write_settings(settings: &std::collections::HashMap<String, String>) {
    if let Ok(data) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(settings_path(), data);
    }
}

#[tauri::command]
pub fn get_setting(key: String) -> Option<String> {
    read_settings().get(&key).cloned()
}

#[tauri::command]
pub fn set_setting(key: String, value: String) {
    let mut settings = read_settings();
    settings.insert(key, value);
    write_settings(&settings);
}

// --- Devices ---

#[derive(serde::Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub api: String,
    pub sample_rate: u32,
    pub is_default: bool,
}

#[tauri::command]
pub fn get_audio_devices() -> Result<Vec<DeviceInfo>, String> {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    let default = host.default_output_device();
    let mut devices = Vec::new();

    for device in host.output_devices().map_err(|e| e.to_string())? {
        let name = device.name().unwrap_or_else(|_| "Unknown".into());
        let config = device.default_output_config().ok();
        let sample_rate = config.as_ref().map(|c| c.sample_rate().0).unwrap_or(0);
        let is_default = default.as_ref().map(|d| {
            d.name().ok() == Some(name.clone())
        }).unwrap_or(false);
        devices.push(DeviceInfo {
            name,
            api: "WASAPI".into(),
            sample_rate,
            is_default,
        });
    }

    Ok(devices)
}

// --- Spotify Integration ---

#[tauri::command]
pub async fn start_oauth_server() -> Result<String, String> {
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let listener = TcpListener::bind("127.0.0.1:1424").await.map_err(|e| e.to_string())?;
    
    // Accept a single connection
    if let Ok((mut stream, _)) = listener.accept().await {
        let mut buffer = [0; 4096];
        if stream.read(&mut buffer).await.is_ok() {
            let request = String::from_utf8_lossy(&buffer);
            
            if let Some(first_line) = request.lines().next() {
                if first_line.starts_with("GET /callback") {
                    let response_html = "HTTP/1.1 200 OK\r\n\
                                         Content-Type: text/html\r\n\
                                         \r\n\
                                         <html><body style='font-family: monospace; background: #222; color: #fff; padding: 2rem;'>\
                                         <h2>// rifly</h2>\
                                         <p>spotify connected. you can close this window.</p>\
                                         <script>setTimeout(() => window.close(), 1000)</script>\
                                         </body></html>";
                    let _ = stream.write_all(response_html.as_bytes()).await;
                    
                    if let Some(query_start) = first_line.find('?') {
                        if let Some(query_end) = first_line.find(" HTTP") {
                            let query = &first_line[query_start + 1..query_end];
                            for param in query.split('&') {
                                if param.starts_with("code=") {
                                    return Ok(param[5..].to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Err("Failed to capture code".into())
}
