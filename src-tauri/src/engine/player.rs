use crate::domain::audio::PlaybackState;
use crate::domain::track::Track;
use crate::engine::state::PlaybackController;
use crate::infrastructure::metadata;

pub struct Player {
    pub controller: PlaybackController,
    current_track: Option<Track>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            controller: PlaybackController::new(),
            current_track: None,
        }
    }

    pub fn play(&mut self, path: &str) -> Result<(), String> {
        let p = std::path::Path::new(path);
        let track = metadata::read_metadata(p)?;
        self.current_track = Some(track);
        self.controller.set_state(PlaybackState::Playing);
        self.controller.set_position(0.0);
        self.controller.set_duration(self.current_track.as_ref().unwrap().duration);
        Ok(())
    }

    pub fn pause(&mut self) {
        if matches!(self.controller.get_state(), PlaybackState::Playing) {
            self.controller.set_state(PlaybackState::Paused);
        }
    }

    pub fn resume(&mut self) {
        if matches!(self.controller.get_state(), PlaybackState::Paused) {
            self.controller.set_state(PlaybackState::Playing);
        }
    }

    pub fn stop(&mut self) {
        self.controller.set_state(PlaybackState::Stopped);
        self.controller.set_position(0.0);
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.current_track.as_ref()
    }
}
