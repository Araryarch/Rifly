mod commands;
mod domain;
mod engine;
mod infrastructure;

use std::sync::Mutex;

use commands::AppState;
use engine::player::Player;
use tauri::{Manager, Emitter};

fn start_position_emitter(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(200));
            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(player) = state.player.lock() {
                    let status = match player.controller.get_state() {
                        crate::domain::audio::PlaybackState::Playing => "playing",
                        crate::domain::audio::PlaybackState::Paused => "paused",
                        crate::domain::audio::PlaybackState::Stopped => "stopped",
                        crate::domain::audio::PlaybackState::Loading => "loading",
                    };
                    let _ = app.emit("player:state", serde_json::json!({
                        "status": status,
                        "position": player.controller.get_position(),
                        "duration": player.controller.get_duration(),
                    }));
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState {
                player: Mutex::new(Player::new()),
            });

            let handle = app.handle().clone();
            start_position_emitter(handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::play_track,
            commands::pause_track,
            commands::resume_track,
            commands::stop_track,
            commands::scan_folder,
            commands::read_track_metadata,
            commands::get_cover_art,
            commands::get_audio_devices,
            commands::get_setting,
            commands::set_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
