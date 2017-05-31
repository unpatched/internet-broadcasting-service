use av::ffi::AVRational;
use portaudio_rs;

use portaudio_rs::stream::FRAMES_PER_BUFFER_UNSPECIFIED;

#[derive(Debug, Copy, Clone)]
pub struct PortAudioConfig {
    av_rational:    AVRational,
    sample_rate:    f64,
    interleaved:    bool,
    input_channel:  u32,
    output_channel: u32,
//  Use 0 for unspecified (lets portaudio pick the proper frame size)
    frames_per_buf: u64,
}

impl PortAudioConfig {
    pub fn new(av_rational:    AVRational,
               sample_rate:    f64,
               interleaved:    bool,
               input_channel:  u32,
               output_channel: u32,
               frames_per_buf: u64) -> PortAudioConfig {
        PortAudioConfig {
            av_rational,
            sample_rate,
            interleaved,
            input_channel,
            output_channel,
            frames_per_buf,
        }
    }
    pub fn av_rational(&self) -> AVRational {
        self.av_rational
    }
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }
    pub fn interleaved(&self) -> bool {
        self.interleaved
    }
    pub fn input_channel(&self) -> u32 {
        self.input_channel
    }
    pub fn output_channel(&self) -> u32 {
        self.output_channel
    }
    pub fn frames_per_buf(&self) -> u64 {
        self.frames_per_buf
    }
}

pub enum PortAudioRuntimeState {
    None,
}

pub struct PortAudio {
    configuration: PortAudioConfig,
    pub runtime_state: PortAudioRuntimeState,
}

impl PortAudio {
    pub fn new(config: PortAudioConfig) -> PortAudio {
        PortAudio {
            configuration: config.clone(),
            runtime_state: PortAudioRuntimeState::None,
        }
    }
}
