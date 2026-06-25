use crate::SDL_AudioDeviceID;

/// A value used to request a default recording audio device.
///
/// Several functions that require an [`SDL_AudioDeviceID`] will accept this value to signify the
/// app just wants the system to choose a default device instead of the app providing a specific
/// one.
pub const SDL_AUDIO_DEVICE_DEFAULT_RECORDING: SDL_AudioDeviceID = 0xFFFFFFFE;

/// A value used to request a default playback audio device.
///
/// Several functions that require an [`SDL_AudioDeviceID`] will accept this value to signify the
/// app just wants the system to choose a default device instead of the app providing a specific
/// one.
pub const SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK: SDL_AudioDeviceID = 0xFFFFFFFF;
