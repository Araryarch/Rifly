use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SampleFormat {
    F32,
    I32,
    I24,
    I16,
    I8,
    U8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OutputApi {
    WasapiShared,
    WasapiExclusive,
    Asio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Loading,
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub bit_depth: u32,
    pub channels: u16,
    pub codec: String,
    pub bitrate: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioSignalPath {
    pub source: String,
    pub decoder: String,
    pub dsp: Vec<String>,
    pub output: String,
    pub device: String,
    pub bit_perfect: bool,
    pub exclusive: bool,
    pub native_dsd: bool,
    pub replaygain: bool,
    pub resampled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DsdMode {
    Native,
    Dop,
    None,
}
