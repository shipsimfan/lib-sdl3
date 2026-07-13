use crate::SDL_AudioFormat;
use std::ffi::c_int;

/// Format specifier for audio data.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct SDL_AudioSpec {
    /// Audio data format
    pub format: SDL_AudioFormat,

    /// Number of channels: 1 mono, 2 stereo, etc
    pub channels: c_int,

    /// Sample rate: sample frames per second
    pub freq: c_int,
}

impl Default for SDL_AudioSpec {
    fn default() -> Self {
        SDL_AudioSpec {
            format: SDL_AudioFormat::Unknown,
            channels: 0,
            freq: 0,
        }
    }
}
