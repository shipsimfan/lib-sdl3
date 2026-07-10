use crate::SDL_AudioSpec;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_free};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Loads a WAV from a file path.
    ///
    /// # Parameters
    ///  * `path` - the file path of the WAV file to open.
    ///  * `spec` - a pointer to an [`SDL_AudioSpec`] that will be set to the WAVE data's format
    ///             details on successful return.
    ///  * `audio_buf` - a pointer filled with the audio data, allocated by the function.
    ///  * `audio_len` - a pointer filled with the length of the audio data buffer in bytes.
    ///
    /// # Return Value
    /// Returns `true` on success. `audio_buf` will be filled with a pointer to an allocated buffer
    /// containing the audio data, and `audio_len` is filled with the length of that audio buffer
    /// in bytes.
    ///
    /// This function returns `false` if the .WAV file cannot be opened, uses an unknown data
    /// format, or is corrupt; call [`SDL_GetError`] for more information.
    ///
    /// When the application is done with the data returned in `audio_buf`, it should call
    /// [`SDL_free`] to dispose of it.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_LoadWAV(
        path: *const c_char,
        spec: *mut SDL_AudioSpec,
        audio_buf: *mut *mut u8,
        audio_len: *mut u32,
    ) -> bool;
}
