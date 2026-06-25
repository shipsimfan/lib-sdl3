use crate::SDL_AudioStream;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Clear any pending data in the stream.
    ///
    /// # Parameters
    ///  * `stream` - the audio stream to clear.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This drops any queued data, so there will be nothing to read from the stream until more is
    /// added.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_ClearAudioStream(stream: *mut SDL_AudioStream) -> bool;
}
