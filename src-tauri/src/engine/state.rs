use crate::domain::audio::PlaybackState;
use crate::domain::playlist::RepeatMode;
use std::sync::{Arc, Mutex};

pub struct PlaybackController {
    pub state: Arc<Mutex<PlaybackState>>,
    pub position: Arc<Mutex<f64>>,
    pub duration: Arc<Mutex<f64>>,
    pub volume: Arc<Mutex<f32>>,
    pub shuffle: Arc<Mutex<bool>>,
    pub repeat: Arc<Mutex<RepeatMode>>,
}

impl PlaybackController {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PlaybackState::Stopped)),
            position: Arc::new(Mutex::new(0.0)),
            duration: Arc::new(Mutex::new(0.0)),
            volume: Arc::new(Mutex::new(0.7)),
            shuffle: Arc::new(Mutex::new(false)),
            repeat: Arc::new(Mutex::new(RepeatMode::Off)),
        }
    }

    pub fn set_position(&self, pos: f64) {
        *self.position.lock().unwrap() = pos;
    }

    pub fn get_position(&self) -> f64 {
        *self.position.lock().unwrap()
    }

    pub fn set_duration(&self, dur: f64) {
        *self.duration.lock().unwrap() = dur;
    }

    pub fn get_duration(&self) -> f64 {
        *self.duration.lock().unwrap()
    }

    pub fn set_state(&self, s: PlaybackState) {
        *self.state.lock().unwrap() = s;
    }

    pub fn get_state(&self) -> PlaybackState {
        *self.state.lock().unwrap()
    }
}
