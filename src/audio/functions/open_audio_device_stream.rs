use crate::{SDL_AudioDeviceID, SDL_AudioSpec, SDL_AudioStream, SDL_AudioStreamCallback};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK, SDL_AUDIO_DEVICE_DEFAULT_RECORDING, SDL_DestroyAudioStream,
    SDL_GetAudioStreamData, SDL_GetAudioStreamFormat, SDL_GetError, SDL_ResumeAudioStreamDevice,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Convenience function for straightforward audio init for the common case.
    ///
    /// # Parameters
    ///  * `devid` - an audio device to open, or [`SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK`] or
    ///              [`SDL_AUDIO_DEVICE_DEFAULT_RECORDING`].
    ///  * `spec` - the audio stream's data format. Can be [`null`].
    ///  * `callback` - a callback where the app will provide new data for playback, or receive new
    ///                 data for recording. Can be [`None`], in which case the app will need to
    ///                 call [`SDL_PutAudioStreamData`] or [`SDL_GetAudioStreamData`] as necessary.
    ///  * `userdata` - app-controlled pointer passed to callback. Can be [`null_mut`]. Ignored if
    ///                 callback is [`None`].
    ///
    /// # Return Value
    /// Returns an audio stream on success, ready to use, or [`null_mut`] on failure; call
    /// [`SDL_GetError`] for more information. When done with this stream, call
    /// [`SDL_DestroyAudioStream`] to free resources and close the device.
    ///
    /// # Remarks
    /// If all your app intends to do is provide a single source of PCM audio, this function allows
    /// you to do all your audio setup in a single call.
    ///
    /// This is also intended to be a clean means to migrate apps from SDL2.
    ///
    /// This function will open an audio device, create a stream and bind it. Unlike other methods
    /// of setup, the audio device will be closed when this stream is destroyed, so the app can
    /// treat the returned [`SDL_AudioStream`] as the only object needed to manage audio playback.
    ///
    /// Also unlike other functions, the audio device begins paused. This is to map more closely to
    /// SDL2-style behavior, since there is no extra step here to bind a stream to begin audio
    /// flowing. The audio device should be resumed with [`SDL_ResumeAudioStreamDevice`].
    ///
    /// This function works with both playback and recording devices.
    ///
    /// The spec parameter represents the app's side of the audio stream. That is, for recording
    /// audio, this will be the output format, and for playing audio, this will be the input
    /// format. If spec is [`null`], the system will choose the format, and the app can use
    /// [`SDL_GetAudioStreamFormat`] to obtain this information later.
    ///
    /// If you don't care about opening a specific audio device, you can (and probably should), use
    /// [`SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK`] for playback and
    /// [`SDL_AUDIO_DEVICE_DEFAULT_RECORDING`] for recording.
    ///
    /// One can optionally provide a callback function; if [`None`], the app is expected to queue
    /// audio data for playback (or unqueue audio data if capturing). Otherwise, the callback will
    /// begin to fire once the device is unpaused.
    ///
    /// Destroying the returned stream with [`SDL_DestroyAudioStream`] will also close the audio
    /// device associated with this stream.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_OpenAudioDeviceStream(
        devid: SDL_AudioDeviceID,
        spec: *const SDL_AudioSpec,
        callback: Option<SDL_AudioStreamCallback>,
        userdata: *mut c_void,
    ) -> *mut SDL_AudioStream;
}
