use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};

use crate::domain::audio::PlaybackState;
use crate::domain::track::Track;
use crate::engine::player::Player;

const POLL_MS: u64 = 2000;
const RECONNECT_BASE_MS: u64 = 2000;
const RECONNECT_MAX_MS: u64 = 30_000;

pub struct DiscordService {
    shutdown: Arc<AtomicBool>,
}

impl Drop for DiscordService {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

impl DiscordService {
    pub fn new(player: Arc<Mutex<Player>>, settings_path: std::path::PathBuf) -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let s = shutdown.clone();
        thread::Builder::new()
            .name("discord-rpc".into())
            .spawn(move || run_loop(player, settings_path, s))
            .ok();
        Self { shutdown }
    }
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn quality_text(t: &Track) -> String {
    if t.is_dsd() {
        let label = if t.sample_rate >= 11289000 { "DSD512" }
        else if t.sample_rate >= 5645000 { "DSD256" }
        else if t.sample_rate >= 2822000 { "DSD128" }
        else if t.sample_rate >= 1411000 { "DSD64" }
        else { "DSD" };
        format!("{} Native", label)
    } else {
        let fmt = t.format.to_uppercase();
        let khz = t.sample_rate as f64 / 1000.0;
        let khz_s = if khz.fract() == 0.0 {
            format!("{}kHz", khz as i32)
        } else {
            format!("{:.1}kHz", khz)
        };
        if t.bit_depth > 0 {
            format!("{} • {}-bit • {}", fmt, t.bit_depth, khz_s)
        } else {
            format!("{} • {}", fmt, khz_s)
        }
    }
}

fn small_image(t: &Track) -> &'static str {
    match t.format.to_lowercase().as_str() {
        "flac" => "flac",
        "alac" | "m4a" => "alac",
        "wav" => "wav",
        "aiff" | "aif" | "aifc" => "aiff",
        "dsf" | "dff" | "dsd" => "dsd",
        "mp3" => "mp3",
        "aac" | "m4b" | "m4p" => "aac",
        "ogg" | "opus" => "opus",
        "wma" => "wma",
        "ape" => "ape",
        "wv" | "wavpack" => "wavpack",
        "tak" => "tak",
        _ => "music",
    }
}

fn small_hover(t: &Track) -> String {
    let base = quality_text(t);
    if t.channels >= 6 {
        format!("{} • {}.1ch", base, t.channels - 1)
    } else if t.channels == 1 {
        format!("{} • Mono", base)
    } else {
        base
    }
}

fn get_settings(path: &std::path::Path) -> HashMap<String, String> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn build_activity<'a>(
    t: &'a Track,
    pos: f64,
    dur: f64,
    is_paused: bool,
    settings: &'a HashMap<String, String>,
) -> activity::Activity<'a> {
    let show_title = settings.get("discord_show_title").map(|v| v == "true").unwrap_or(true);
    let show_artist = settings.get("discord_show_artist").map(|v| v == "true").unwrap_or(true);
    let show_album = settings.get("discord_show_album").map(|v| v == "true").unwrap_or(true);
    let show_quality = settings.get("discord_show_audio_quality").map(|v| v == "true").unwrap_or(true);
    let show_progress = settings.get("discord_show_playback_progress").map(|v| v == "true").unwrap_or(true);
    let audiophile = settings.get("discord_audiophile_mode").map(|v| v == "true").unwrap_or(false);

    let mut act = activity::Activity::new()
        .activity_type(activity::ActivityType::Listening);

    if audiophile {
        let (first, second) = match (show_title, show_artist) {
            (true, true) => (
                Some(format!("{} — {}", t.artist, t.title)),
                Some(format!("{} • Bit Perfect ✓", quality_text(t))),
            ),
            (true, false) => (
                Some(t.title.clone()),
                Some(format!("{} • Bit Perfect ✓", quality_text(t))),
            ),
            (false, true) => (
                Some(t.artist.clone()),
                Some(format!("{} • Bit Perfect ✓", quality_text(t))),
            ),
            (false, false) => (None, Some(format!("{} • Bit Perfect ✓", quality_text(t)))),
        };
        if let Some(d) = first { act = act.details(d); }
        if let Some(s) = second { act = act.state(s); }
    } else {
        if show_title { act = act.details(&t.title); }
        if show_artist {
            act = if show_title { act.state(&t.artist) } else { act.details(&t.artist) };
        }
        if show_quality && !show_title && !show_artist {
            act = act.details(quality_text(t));
        }
    }

    let mut assets_payload = activity::Assets::new()
        .large_image("rifly_logo")
        .large_text(if show_album { &t.album } else { "Rifly" });

    if show_quality {
        assets_payload = assets_payload
            .small_image(small_image(t))
            .small_text(small_hover(t));
    }

    act = act.assets(assets_payload);

    if show_progress && !is_paused {
        let now = now_unix();
        let start = now - pos as i64;
        let end = start + dur as i64;
        act = act.timestamps(activity::Timestamps::new().start(start).end(end));
    }

    let b1_label = settings.get("discord_button1_label").map(|s| s.as_str()).unwrap_or("");
    let b1_url = settings.get("discord_button1_url").map(|s| s.as_str()).unwrap_or("");
    let b2_label = settings.get("discord_button2_label").map(|s| s.as_str()).unwrap_or("");
    let b2_url = settings.get("discord_button2_url").map(|s| s.as_str()).unwrap_or("");

    let mut buttons = Vec::new();
    if !b1_label.is_empty() && !b1_url.is_empty() {
        buttons.push(activity::Button::new(b1_label, b1_url));
    }
    if !b2_label.is_empty() && !b2_url.is_empty() {
        buttons.push(activity::Button::new(b2_label, b2_url));
    }
    if !buttons.is_empty() {
        act = act.buttons(buttons);
    }

    act
}

fn run_loop(player: Arc<Mutex<Player>>, settings_path: std::path::PathBuf, shutdown: Arc<AtomicBool>) {
    let mut client: Option<DiscordIpcClient> = None;
    let mut last_track_path = String::new();
    let mut last_state = PlaybackState::Stopped;
    let mut last_quality = String::new();
    let mut last_settings_raw = String::new();
    let mut delay = RECONNECT_BASE_MS;

    while !shutdown.load(Ordering::SeqCst) {
        let settings_raw = std::fs::read_to_string(&settings_path).unwrap_or_default();
        let map: HashMap<String, String> = serde_json::from_str(&settings_raw).unwrap_or_default();
        let enabled = map.get("discord_enabled").map(|v| v == "true").unwrap_or(true);
        let client_id = map.get("discord_client_id").cloned().unwrap_or_default();
        let hide_all = map.get("discord_hide_everything").map(|v| v == "true").unwrap_or(false);

        let should_connect = enabled && !client_id.is_empty();

        if client.is_none() && should_connect {
            let mut c = DiscordIpcClient::new(&client_id);
            match c.connect() {
                Ok(()) => {
                    client = Some(c);
                    delay = RECONNECT_BASE_MS;
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(delay));
                    delay = (delay * 2).min(RECONNECT_MAX_MS);
                    continue;
                }
            }
        }

        if let Some(ref mut c) = client {
            let (track, pb_state, pos, dur) = match player.lock() {
                Ok(p) => {
                    let st = p.controller.get_state();
                    (p.current_track().cloned(), st, p.controller.get_position(), p.controller.get_duration())
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
            };

            let needs_clear = !should_connect
                || pb_state == PlaybackState::Stopped
                || track.is_none()
                || hide_all;

            if needs_clear {
                if last_track_path != "CLEARED" {
                    let _ = c.clear_activity();
                    last_track_path = "CLEARED".into();
                    last_state = PlaybackState::Stopped;
                }
            } else {
                let t = track.as_ref().unwrap();
                let qual = quality_text(t);
                let track_changed = t.path != last_track_path;
                let state_changed = pb_state != last_state;
                let quality_changed = qual != last_quality;
                let settings_changed = settings_raw != last_settings_raw;

                if track_changed || state_changed || quality_changed || settings_changed {
                    let act = build_activity(t, pos, dur, pb_state == PlaybackState::Paused, &map);
                    match c.set_activity(act) {
                        Ok(()) => {
                            last_track_path = t.path.clone();
                            last_state = pb_state;
                            last_quality = qual;
                            last_settings_raw = settings_raw;
                        }
                        Err(_) => {
                            let _ = c.close();
                            client = None;
                            delay = RECONNECT_BASE_MS;
                            continue;
                        }
                    }
                }
            }
        } else if !should_connect {
            thread::sleep(Duration::from_millis(5000));
            continue;
        }

        thread::sleep(Duration::from_millis(POLL_MS));
    }

    if let Some(mut c) = client {
        let _ = c.clear_activity();
        let _ = c.close();
    }
}
