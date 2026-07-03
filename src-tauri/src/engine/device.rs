use crate::domain::audio::OutputApi;
use crate::engine::output::{AudioOutput, OutputDeviceInfo};
use cpal::traits::{DeviceTrait, HostTrait};

pub struct WasapiOutput {
    device_name: String,
    exclusive: bool,
    volume: f32,
}

impl WasapiOutput {
    pub fn new(exclusive: bool) -> Self {
        let host = cpal::default_host();
        let name = host
            .default_output_device()
            .map(|d| d.name().unwrap_or_default())
            .unwrap_or_default();

        Self {
            device_name: name,
            exclusive,
            volume: 0.7,
        }
    }
}

impl AudioOutput for WasapiOutput {
    fn name(&self) -> &str {
        &self.device_name
    }

    fn api(&self) -> OutputApi {
        if self.exclusive {
            OutputApi::WasapiExclusive
        } else {
            OutputApi::WasapiShared
        }
    }

    fn open(&mut self, _format: &crate::domain::audio::AudioFormat) -> Result<(), String> {
        Ok(())
    }

    fn write(&mut self, _samples: &[f32], _format: &crate::domain::audio::AudioFormat) -> Result<(), String> {
        Ok(())
    }

    fn close(&mut self) {}

    fn pause(&mut self) {}

    fn resume(&mut self) {}

    fn volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.0);
    }

    fn device_info(&self) -> OutputDeviceInfo {
        OutputDeviceInfo {
            name: self.device_name.clone(),
            manufacturer: String::new(),
            api: self.api(),
            current_sample_rate: 44100,
            max_sample_rate: 384000,
            bit_depth: 32,
            exclusive: self.exclusive,
            asio_available: false,
            dsd_capable: false,
            is_default: true,
        }
    }
}

pub struct DeviceManager {
    outputs: Vec<Box<dyn AudioOutput>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }

    pub fn detect_devices(&mut self) {
        self.outputs.clear();

        let shared = WasapiOutput::new(false);
        let exclusive = WasapiOutput::new(true);
        self.outputs.push(Box::new(shared));
        self.outputs.push(Box::new(exclusive));
    }

    pub fn devices(&self) -> &[Box<dyn AudioOutput>] {
        &self.outputs
    }

    pub fn device_count(&self) -> usize {
        self.outputs.len()
    }
}
