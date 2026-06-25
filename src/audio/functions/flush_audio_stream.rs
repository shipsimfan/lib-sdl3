use crate::SDL_AudioStream;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Tell the stream that you're done sending data, and anything being buffered should be
    /// converted/resampled and made available immediately.
    ///
    /// # Parameters
    ///  * `stream` - the audio stream to flush.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// It is legal to add more data to a stream after flushing, but there may be audio gaps in the
    /// output. Generally this is intended to signal the end of input, so the complete output
    /// becomes available.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_FlushAudioStream(stream: *mut SDL_AudioStream) -> bool;
}
