use crate::SDL_AudioStream;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetAudioStreamData, SDL_GetError};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the number of converted/resampled bytes available.
    ///
    /// # Parameters
    ///  * `stream` - the audio stream to query.
    ///
    /// # Return Value
    /// Returns the number of converted/resampled bytes available or -1 on failure; call
    /// [`SDL_GetError`] for more information.
    ///
    /// # Remarks
    /// The stream may be buffering data behind the scenes until it has enough to resample
    /// correctly, so this number might be lower than what you expect, or even be zero. Add more
    /// data or flush the stream if you need the data now.
    ///
    /// If the stream has so much data that it would overflow an int, the return value is clamped
    /// to a maximum value, but no queued data is lost; if there are gigabytes of data queued, the
    /// app might need to read some of it with [`SDL_GetAudioStreamData`] before this function's
    /// return value is no longer clamped.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetAudioStreamAvailable(stream: *mut SDL_AudioStream) -> c_int;
}
