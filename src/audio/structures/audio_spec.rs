use crate::SDL_AudioFormat;
use std::ffi::c_int;

/// Format specifier for audio data.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_AudioSpec {
    /// Audio data format
    format: SDL_AudioFormat,

    /// Number of channels: 1 mono, 2 stereo, etc
    channels: c_int,

    /// Sample rate: sample frames per second
    freq: c_int,
}
