use crate::domain::audio::{AudioFormat, OutputApi};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OutputDeviceInfo {
    pub name: String,
    pub manufacturer: String,
    pub api: OutputApi,
    pub current_sample_rate: u32,
    pub max_sample_rate: u32,
    pub bit_depth: u32,
    pub exclusive: bool,
    pub asio_available: bool,
    pub dsd_capable: bool,
    pub is_default: bool,
}

pub trait AudioOutput: Send {
    fn name(&self) -> &str;
    fn api(&self) -> OutputApi;
    fn open(&mut self, format: &AudioFormat) -> Result<(), String>;
    fn write(&mut self, samples: &[f32], format: &AudioFormat) -> Result<(), String>;
    fn close(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn volume(&self) -> f32;
    fn set_volume(&mut self, vol: f32);
    fn device_info(&self) -> OutputDeviceInfo;
}

pub struct OutputManager {
    outputs: Vec<Box<dyn AudioOutput>>,
    active: Option<usize>,
}

impl OutputManager {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
            active: None,
        }
    }

    pub fn register(&mut self, output: Box<dyn AudioOutput>) {
        self.outputs.push(output);
    }

    pub fn set_active(&mut self, index: usize) -> Result<(), String> {
        if index < self.outputs.len() {
            if let Some(prev) = self.active {
                self.outputs[prev].close();
            }
            self.active = Some(index);
            Ok(())
        } else {
            Err("Invalid output index".to_string())
        }
    }

    pub fn active_output(&mut self) -> Option<&mut Box<dyn AudioOutput>> {
        self.active.map(|i| &mut self.outputs[i])
    }

    pub fn write(&mut self, samples: &[f32], format: &AudioFormat) -> Result<(), String> {
        if let Some(idx) = self.active {
            self.outputs[idx].write(samples, format)
        } else {
            Err("No active output".to_string())
        }
    }

    pub fn pause(&mut self) {
        if let Some(idx) = self.active {
            self.outputs[idx].pause();
        }
    }

    pub fn resume(&mut self) {
        if let Some(idx) = self.active {
            self.outputs[idx].resume();
        }
    }

    pub fn device_info(&self) -> Option<OutputDeviceInfo> {
        self.active.map(|i| self.outputs[i].device_info())
    }

    pub fn all_devices(&self) -> Vec<OutputDeviceInfo> {
        self.outputs.iter().map(|o| o.device_info()).collect()
    }
}
