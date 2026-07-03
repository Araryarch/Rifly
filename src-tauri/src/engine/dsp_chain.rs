use crate::domain::dsp::{DspChain, DspProcessor, DspType};

pub struct VolumeProcessor {
    volume: f32,
    enabled: bool,
}

impl VolumeProcessor {
    pub fn new(volume: f32) -> Self {
        Self {
            volume: volume.clamp(0.0, 1.0),
            enabled: true,
        }
    }
}

impl DspProcessor for VolumeProcessor {
    fn name(&self) -> &str {
        "Volume"
    }
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32, _channels: u16) {
        if self.volume < 1.0 {
            for s in samples.iter_mut() {
                *s *= self.volume;
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
        DspType::Volume
    }
}

pub struct ReplayGainProcessor {
    gain: f64,
    enabled: bool,
}

impl ReplayGainProcessor {
    pub fn new(gain: f64) -> Self {
        Self {
            gain,
            enabled: true,
        }
    }
}

impl DspProcessor for ReplayGainProcessor {
    fn name(&self) -> &str {
        "ReplayGain"
    }
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32, _channels: u16) {
        let scale = (10.0_f64).powf(self.gain / 20.0) as f32;
        if (scale - 1.0).abs() > 0.001 {
            for s in samples.iter_mut() {
                *s *= scale;
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
        DspType::ReplayGain
    }
}

pub fn create_default_dsp_chain(volume: f32) -> DspChain {
    let mut chain = DspChain::new();
    chain.add(Box::new(VolumeProcessor::new(volume)));
    chain
}
