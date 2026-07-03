use crate::domain::dsp::{DspProcessor, DspType};

pub struct CrossfadeProcessor {
    pub duration_secs: f32,
    enabled: bool,
}

impl CrossfadeProcessor {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            duration_secs,
            enabled: false,
        }
    }
}

impl DspProcessor for CrossfadeProcessor {
    fn name(&self) -> &str {
        "Crossfade"
    }

    fn process(&mut self, _samples: &mut [f32], _sample_rate: u32, _channels: u16) {}

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn dsp_type(&self) -> DspType {
        DspType::Crossfade
    }
}
