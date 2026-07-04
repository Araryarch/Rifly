#![allow(dead_code)]
pub mod commands;
mod domain;
mod engine;
mod infrastructure;
mod services;

use std::sync::{Arc, Mutex};

use commands::AppState;
use engine::player::Player;
use services::discord::DiscordService;
use tauri::{Manager, Emitter};
use tauri::menu::{Menu, MenuItem};

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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let player = Arc::new(Mutex::new(Player::new()));

            let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
            std::fs::create_dir_all(&data_dir).ok();

            let _discord = DiscordService::new(player.clone(), data_dir.clone());

            app.manage(AppState {
                player,
                _discord: Mutex::new(_discord),
                data_dir,
            });

            let handle = app.handle().clone();
            start_position_emitter(handle.clone());

            // --- System Tray ---
            let tray_icon = app.default_window_icon().cloned()
                .expect("No default window icon configured");

            let show_item = MenuItem::with_id(&handle, "show", "Show", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(&handle, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(&handle, &[&show_item, &quit_item])?;

            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(tray_icon)
                .tooltip("Rifly")
                .menu(&tray_menu)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

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
            commands::toggle_favorite,
            commands::get_favorites,
            commands::log_play,
            commands::get_recently_played,
            commands::get_most_played,
            commands::save_session,
            commands::load_session,
            commands::start_oauth_server,
            commands::add_music_files,
            commands::get_imported_files,
            commands::remove_imported_file,
            commands::edit_track_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
