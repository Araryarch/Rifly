use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::path::PathBuf;
use crate::domain::track::Track;
use crate::infrastructure::metadata;
use crate::engine::player::Player;
use crate::services::discord::DiscordService;

pub struct AppState {
    pub player: Arc<Mutex<Player>>,
    pub _discord: Mutex<DiscordService>,
    pub data_dir: PathBuf,
}

// --- Helpers ---

fn settings_path(state: &AppState) -> PathBuf {
    state.data_dir.join("rifly_settings.json")
}

fn favorites_path(state: &AppState) -> PathBuf {
    state.data_dir.join("rifly_favorites.json")
}

fn history_path(state: &AppState) -> PathBuf {
    state.data_dir.join("rifly_history.json")
}

fn session_path(state: &AppState) -> PathBuf {
    state.data_dir.join("rifly_session.json")
}

fn imported_path(state: &AppState) -> PathBuf {
    state.data_dir.join("rifly_imported.json")
}

fn read_json<T: serde::de::DeserializeOwned + Default>(path: &PathBuf) -> T {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn write_json<T: serde::Serialize>(path: &PathBuf, data: &T) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = std::fs::write(path, json);
    }
}

fn read_settings(state: &AppState) -> std::collections::HashMap<String, String> {
    read_json(&settings_path(state))
}

fn write_settings(state: &AppState, settings: &std::collections::HashMap<String, String>) {
    write_json(&settings_path(state), settings);
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

// --- Settings ---

#[tauri::command]
pub fn get_setting(state: tauri::State<AppState>, key: String) -> Option<String> {
    read_settings(&state).get(&key).cloned()
}

#[tauri::command]
pub fn set_setting(state: tauri::State<AppState>, key: String, value: String) {
    let mut settings = read_settings(&state);
    settings.insert(key, value);
    write_settings(&state, &settings);
}

// --- Favorites ---

fn read_favorites(state: &AppState) -> HashSet<String> {
    read_json::<Vec<String>>(&favorites_path(state))
        .into_iter()
        .collect()
}

fn write_favorites(state: &AppState, favorites: &HashSet<String>) {
    let vec: Vec<&String> = favorites.iter().collect();
    write_json(&favorites_path(state), &vec);
}

#[tauri::command]
pub fn toggle_favorite(state: tauri::State<AppState>, path: String) -> Result<bool, String> {
    let mut favorites = read_favorites(&state);
    let is_fav = !favorites.contains(&path);
    if is_fav { favorites.insert(path); } else { favorites.remove(&path); }
    write_favorites(&state, &favorites);
    Ok(is_fav)
}

#[tauri::command]
pub fn get_favorites(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    Ok(read_favorites(&state).into_iter().collect())
}

// --- Play History ---

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PlayRecord {
    pub path: String,
    pub timestamp: u64,
    pub play_count: u64,
}

fn read_history(state: &AppState) -> std::collections::HashMap<String, PlayRecord> {
    read_json(&history_path(state))
}

fn write_history(state: &AppState, history: &std::collections::HashMap<String, PlayRecord>) {
    write_json(&history_path(state), history);
}

#[tauri::command]
pub fn log_play(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    let mut history = read_history(&state);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
    let record = history.entry(path.clone()).or_insert(PlayRecord { path, timestamp: 0, play_count: 0 });
    record.timestamp = now;
    record.play_count += 1;
    write_history(&state, &history);
    Ok(())
}

#[tauri::command]
pub fn get_recently_played(state: tauri::State<AppState>, limit: Option<usize>) -> Result<Vec<PlayRecord>, String> {
    let mut records: Vec<PlayRecord> = read_history(&state).into_values().collect();
    records.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let limit = limit.unwrap_or(50);
    records.truncate(limit);
    Ok(records)
}

#[tauri::command]
pub fn get_most_played(state: tauri::State<AppState>, limit: Option<usize>) -> Result<Vec<PlayRecord>, String> {
    let mut records: Vec<PlayRecord> = read_history(&state).into_values().collect();
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

#[tauri::command]
pub fn save_session(state: tauri::State<AppState>, track_path: String, position: f64, queue: Vec<String>, queue_index: usize) -> Result<(), String> {
    let session = SessionData { track_path, position, queue, queue_index };
    write_json(&session_path(&state), &session);
    Ok(())
}

#[tauri::command]
pub fn load_session(state: tauri::State<AppState>) -> Result<Option<SessionData>, String> {
    Ok(read_json(&session_path(&state)))
}

// --- Import Music ---

#[tauri::command]
pub fn add_music_files(state: tauri::State<AppState>, paths: Vec<String>) -> Result<Vec<Track>, String> {
    let mut imported: Vec<String> = read_json(&imported_path(&state));
    let mut tracks = Vec::new();
    for p in &paths {
        let path = std::path::Path::new(p);
        if !path.exists() { continue; }
        if imported.contains(p) { continue; }
        match metadata::read_metadata(path) {
            Ok(track) => {
                imported.push(p.clone());
                tracks.push(track);
            }
            Err(e) => eprintln!("Import error for {}: {}", p, e),
        }
    }
    write_json(&imported_path(&state), &imported);
    Ok(tracks)
}

#[tauri::command]
pub fn get_imported_files(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    Ok(read_json(&imported_path(&state)))
}

#[tauri::command]
pub fn remove_imported_file(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    let mut imported: Vec<String> = read_json(&imported_path(&state));
    imported.retain(|p| p != &path);
    write_json(&imported_path(&state), &imported);
    Ok(())
}

// --- Edit Metadata ---

#[tauri::command]
pub fn edit_track_metadata(
    path: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    genre: Option<String>,
    year: Option<i32>,
    track_number: Option<i32>,
) -> Result<Track, String> {
    let p = std::path::Path::new(&path);
    metadata::write_metadata(&p, title, artist, album, album_artist, genre, year, track_number)?;
    metadata::read_metadata(&p)
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
        let is_default = default.as_ref().map(|d| d.name().ok() == Some(name.clone())).unwrap_or(false);
        devices.push(DeviceInfo { name, api: "WASAPI".into(), sample_rate, is_default });
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
                        Content-Type: text/html\r\n\r\n\
                        <html><body style='font-family:monospace;background:#222;color:#fff;padding:2rem;'>\
                        <h2>// rifly</h2>\
                        <p>spotify connected. you can close this window.</p>\
                        <script>setTimeout(()=>window.close(),1000)</script></body></html>";
                    let _ = stream.write_all(response_html.as_bytes()).await;
                    if let Some(qs) = first_line.find('?') {
                        if let Some(qe) = first_line.find(" HTTP") {
                            for param in first_line[qs + 1..qe].split('&') {
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
