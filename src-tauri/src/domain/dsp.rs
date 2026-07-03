use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DspType {
    Volume,
    ReplayGain,
    Equalizer,
    Crossfade,
    Crossfeed,
    Limiter,
    Normalizer,
    BassBoost,
    Balance,
    Mono,
    SwapChannels,
    Convolver,
}

impl DspType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Volume => "Volume",
            Self::ReplayGain => "ReplayGain",
            Self::Equalizer => "Equalizer",
            Self::Crossfade => "Crossfade",
            Self::Crossfeed => "Crossfeed",
            Self::Limiter => "Limiter",
            Self::Normalizer => "Normalizer",
            Self::BassBoost => "Bass Boost",
            Self::Balance => "Balance",
            Self::Mono => "Mono",
            Self::SwapChannels => "Swap Channels",
            Self::Convolver => "Convolver",
        }
    }
}

pub trait DspProcessor: Send {
    fn name(&self) -> &str;
    fn process(&mut self, samples: &mut [f32], sample_rate: u32, channels: u16);
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn dsp_type(&self) -> DspType;
}

pub struct DspChain {
    processors: Vec<Box<dyn DspProcessor>>,
    enabled: bool,
}

impl DspChain {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
            enabled: true,
        }
    }

    pub fn add(&mut self, processor: Box<dyn DspProcessor>) {
        self.processors.push(processor);
    }

    pub fn insert_at(&mut self, index: usize, processor: Box<dyn DspProcessor>) {
        if index <= self.processors.len() {
            self.processors.insert(index, processor);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Box<dyn DspProcessor>> {
        if index < self.processors.len() {
            Some(self.processors.remove(index))
        } else {
            None
        }
    }

    pub fn process(&mut self, samples: &mut [f32], sample_rate: u32, channels: u16) {
        if !self.enabled {
            return;
        }
        for processor in &mut self.processors {
            if processor.enabled() {
                processor.process(samples, sample_rate, channels);
            }
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn len(&self) -> usize {
        self.processors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processors.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Box<dyn DspProcessor>> {
        self.processors.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Box<dyn DspProcessor>> {
        self.processors.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn DspProcessor>> {
        self.processors.iter()
    }

    pub fn active_names(&self) -> Vec<String> {
        self.processors
            .iter()
            .filter(|p| p.enabled())
            .map(|p| p.name().to_string())
            .collect()
    }

    pub fn clear(&mut self) {
        self.processors.clear();
    }
}
