use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub composers: Vec<String>,
    pub genres: Vec<String>,
    pub track_number: i32,
    pub disc_number: i32,
    pub duration: f64,
    pub sample_rate: i32,
    pub bit_depth: i32,
    pub channels: i32,
    pub format: String,
    pub year: i32,
    pub has_artwork: bool,
    pub replaygain_track: f64,
    pub replaygain_album: f64,
}

impl Track {
    pub fn is_hi_res(&self) -> bool {
        self.sample_rate > 48000 || self.bit_depth > 16
    }

    pub fn is_dsd(&self) -> bool {
        matches!(self.format.to_lowercase().as_str(), "dsd" | "dsf" | "dff")
    }

    pub fn is_lossless(&self) -> bool {
        matches!(
            self.format.to_lowercase().as_str(),
            "flac" | "alac" | "wav" | "aiff" | "ape" | "tak" | "wavpack" | "dsf" | "dff" | "dsd"
        )
    }

    pub fn hi_res_badge(&self) -> Option<String> {
        if self.sample_rate > 48000 {
            Some(format!("{}kHz", self.sample_rate / 1000))
        } else if self.bit_depth > 16 {
            Some(format!("{}bit", self.bit_depth))
        } else {
            None
        }
    }
}
