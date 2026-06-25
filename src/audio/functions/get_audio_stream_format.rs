use crate::{SDL_AudioSpec, SDL_AudioStream};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Query the current format of an audio stream.
    ///
    /// # Parameters
    ///  * `stream` - the [`SDL_AudioStream`] to query.
    ///  * `src_spec` - where to store the input audio format; ignored if [`null_mut`].
    ///  * `dst_spec` - where to store the output audio format; ignored if [`null_mut`].
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread, as it holds a stream-specific mutex while
    /// running.
    pub fn SDL_GetAudioStreamFormat(
        stream: *mut SDL_AudioStream,
        src_spec: *mut SDL_AudioSpec,
        dst_spec: *mut SDL_AudioSpec,
    ) -> bool;
}
