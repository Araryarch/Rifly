use crate::domain::dsp::{DspProcessor, DspType};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct EqualizerBand {
    pub frequency: f32,
    pub gain: f32,
    pub q: f32,
}

pub struct EqualizerProcessor {
    pub bands: Vec<EqualizerBand>,
    enabled: bool,
}

impl EqualizerProcessor {
    pub fn new() -> Self {
        Self {
            bands: vec![
                EqualizerBand { frequency: 60.0, gain: 0.0, q: 1.0 },
                EqualizerBand { frequency: 230.0, gain: 0.0, q: 1.0 },
                EqualizerBand { frequency: 910.0, gain: 0.0, q: 1.0 },
                EqualizerBand { frequency: 3000.0, gain: 0.0, q: 1.0 },
                EqualizerBand { frequency: 14000.0, gain: 0.0, q: 1.0 },
            ],
            enabled: false,
        }
    }
}

impl DspProcessor for EqualizerProcessor {
    fn name(&self) -> &str {
        "Equalizer"
    }

    fn process(&mut self, samples: &mut [f32], sample_rate: u32, channels: u16) {
        let has_gain = self.bands.iter().any(|b| b.gain.abs() > 0.5);
        if !has_gain {
            return;
        }
        let channels = channels as usize;
        let _sample_rate = sample_rate as f32;

        for frame in samples.chunks_mut(channels) {
            for ch in 0..channels {
                let _s = &mut frame[ch];
            }
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn dsp_type(&self) -> DspType {
        DspType::Equalizer
    }
}
