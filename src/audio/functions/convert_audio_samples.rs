use crate::SDL_AudioSpec;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_AudioStream, SDL_GetError, SDL_free};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Convert some audio data of one format to another format.
    ///
    /// # Parameters
    ///  * `src_spec` - the format details of the input audio.
    ///  * `src_data` - the audio data to be converted.
    ///  * `src_len` - the len of `src_data`.
    ///  * `dst_spec` - the format details of the output audio.
    ///  * `dst_data` - will be filled with a pointer to converted audio data, which should be
    ///                 freed with [`SDL_free`]. On error, it will be [`null_mut`].
    ///  * `dst_len` - will be filled with the len of `dst_data`.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// Please note that this function is for convenience, but should not be used to resample audio
    /// in blocks, as it will introduce audio artifacts on the boundaries. You should only use this
    /// function if you are converting audio data in its entirety in one call. If you want to
    /// convert audio in smaller chunks, use an [`SDL_AudioStream`], which is designed for this
    /// situation.
    ///
    /// Internally, this function creates and destroys an [`SDL_AudioStream`] on each use, so it's
    /// also less efficient than using one directly, if you need to convert multiple times.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_ConvertAudioSamples(
        src_spec: *const SDL_AudioSpec,
        src_data: *const u8,
        src_len: c_int,
        dst_spec: *const SDL_AudioSpec,
        dst_data: *mut *mut u8,
        dst_len: *mut c_int,
    ) -> bool;
}
