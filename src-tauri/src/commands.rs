use std::sync::{Arc, Mutex, HashSet};
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

// --- Favorites ---

fn favorites_path() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap_or_default();
    p.pop();
    p.push("rifly_favorites.json");
    p
}

fn read_favorites() -> HashSet<String> {
    let path = favorites_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<String>>(&s).ok())
        .map(|v| v.into_iter().collect())
        .unwrap_or_default()
}

fn write_favorites(favorites: &HashSet<String>) {
    let vec: Vec<&String> = favorites.iter().collect();
    if let Ok(data) = serde_json::to_string_pretty(&vec) {
        let _ = std::fs::write(favorites_path(), data);
    }
}

#[tauri::command]
pub fn toggle_favorite(path: String) -> Result<bool, String> {
    let mut favorites = read_favorites();
    let is_fav = !favorites.contains(&path);
    if is_fav {
        favorites.insert(path);
    } else {
        favorites.remove(&path);
    }
    write_favorites(&favorites);
    Ok(is_fav)
}

#[tauri::command]
pub fn get_favorites() -> Result<Vec<String>, String> {
    Ok(read_favorites().into_iter().collect())
}

// --- Play History ---

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PlayRecord {
    pub path: String,
    pub timestamp: u64,
    pub play_count: u64,
}

fn history_path() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap_or_default();
    p.pop();
    p.push("rifly_history.json");
    p
}

fn read_history() -> std::collections::HashMap<String, PlayRecord> {
    let path = history_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn write_history(history: &std::collections::HashMap<String, PlayRecord>) {
    if let Ok(data) = serde_json::to_string_pretty(history) {
        let _ = std::fs::write(history_path(), data);
    }
}

#[tauri::command]
pub fn log_play(path: String) -> Result<(), String> {
    let mut history = read_history();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let record = history.entry(path.clone()).or_insert(PlayRecord {
        path,
        timestamp: 0,
        play_count: 0,
    });
    record.timestamp = now;
    record.play_count += 1;

    write_history(&history);
    Ok(())
}

#[tauri::command]
pub fn get_recently_played(limit: Option<usize>) -> Result<Vec<PlayRecord>, String> {
    let history = read_history();
    let mut records: Vec<PlayRecord> = history.into_values().collect();
    records.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let limit = limit.unwrap_or(50);
    records.truncate(limit);
    Ok(records)
}

#[tauri::command]
pub fn get_most_played(limit: Option<usize>) -> Result<Vec<PlayRecord>, String> {
    let history = read_history();
    let mut records: Vec<PlayRecord> = history.into_values().collect();
    records.sort_by(|a, b| b.play_count.cmp(&a.play_count));
    let limit = limit.unwrap_or(50);
    records.truncate(limit);
    Ok(records)
}

// --- Session Restore ---

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SessionData {
    pub track_path: String,
    pub position: f64,
    pub queue: Vec<String>,
    pub queue_index: usize,
}

fn session_path() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap_or_default();
    p.pop();
    p.push("rifly_session.json");
    p
}

#[tauri::command]
pub fn save_session(track_path: String, position: f64, queue: Vec<String>, queue_index: usize) -> Result<(), String> {
    let session = SessionData { track_path, position, queue, queue_index };
    if let Ok(data) = serde_json::to_string_pretty(&session) {
        let _ = std::fs::write(session_path(), data);
    }
    Ok(())
}

#[tauri::command]
pub fn load_session() -> Result<Option<SessionData>, String> {
    let path = session_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(serde_json::from_str(&content).ok()),
        Err(_) => Ok(None),
    }
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
