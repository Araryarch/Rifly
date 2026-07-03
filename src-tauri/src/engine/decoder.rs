use crate::domain::audio::AudioFormat;
use crate::domain::track::Track;
use std::sync::Arc;

pub struct DecodedAudio {
    pub samples: Arc<Vec<f32>>,
    pub sample_rate: u32,
    pub channels: u16,
    pub format: AudioFormat,
}

pub trait Decoder: Send {
    fn decode(&mut self, path: &str) -> Result<DecodedAudio, String>;
    fn decode_track(&mut self, track: &Track) -> Result<DecodedAudio, String> {
        self.decode(&track.path)
    }
}

pub struct SymphoniaDecoder;

impl Decoder for SymphoniaDecoder {
    fn decode(&mut self, path: &str) -> Result<DecodedAudio, String> {
        use symphonia::core::audio::SampleBuffer;
        use symphonia::core::codecs::DecoderOptions;
        use symphonia::core::formats::FormatOptions;
        use symphonia::core::io::MediaSourceStream;
        use symphonia::core::meta::MetadataOptions;
        use symphonia::core::probe::Hint;

        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        let ext = path.rsplit('.').next().unwrap_or("");
        hint.with_extension(ext);

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
            .map_err(|e| format!("Probe: {e}"))?;

        let mut format = probed.format;
        let track = format.default_track().ok_or("No track")?;
        let params = track.codec_params.clone();

        let track_id = track.id;
        let mut decoder = symphonia::default::get_codecs()
            .make(&params, &DecoderOptions::default())
            .map_err(|e| format!("Codec: {e}"))?;

        let sample_rate = params.sample_rate.unwrap_or(44100);
        let channels = params.channels.map(|c| c.count()).unwrap_or(2) as u16;
        let codec = format!("{:?}", params.codec);
        let bitrate = params.sample_rate.map(|_| 0);

        let mut all_samples: Vec<f32> = Vec::new();
        let mut sample_buf: Option<SampleBuffer<f32>> = None;

        loop {
            let packet = match format.next_packet() {
                Ok(p) => p,
                Err(symphonia::core::errors::Error::IoError(ref e))
                    if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(_) => break,
            };
            if packet.track_id() != track_id {
                continue;
            }
            let decoded = match decoder.decode(&packet) {
                Ok(d) => d,
                Err(_) => continue,
            };
            if decoded.spec().channels.count() != channels as usize {
                continue;
            }
            let n_frames = decoded.frames();
            let spec = *decoded.spec();
            let mut buf = sample_buf.take().unwrap_or_else(|| SampleBuffer::new(0, spec));
            if buf.capacity() < n_frames {
                buf = SampleBuffer::new(n_frames as u64, spec);
            }
            buf.copy_interleaved_ref(decoded);
            all_samples.extend_from_slice(buf.samples());
            sample_buf = Some(buf);
        }

        let _total_frames = all_samples.len() / channels as usize;

        Ok(DecodedAudio {
            samples: Arc::new(all_samples),
            sample_rate,
            channels,
            format: AudioFormat {
                sample_rate,
                bit_depth: params.bits_per_sample.unwrap_or(16) as u32,
                channels,
                codec,
                bitrate,
            },
        })
    }
}
