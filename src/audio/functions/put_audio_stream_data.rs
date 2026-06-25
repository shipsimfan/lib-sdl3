use crate::SDL_AudioStream;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_SetAudioStreamFormat};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Add data to the stream.
    ///
    /// # Parameters
    ///  * `stream` - the stream the audio data is being added to.
    ///  * `buf` - a pointer to the audio data to add.
    ///  * `len` - the number of bytes to write to the stream.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This data must match the format/channels/samplerate specified in the latest call to
    /// [`SDL_SetAudioStreamFormat`], or the format specified when creating the stream if it hasn't
    /// been changed.
    ///
    /// Note that this call simply copies the unconverted data for later. This is different than
    /// SDL2, where data was converted during the Put call and the Get call would just dequeue the
    /// previously-converted data.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread, but if the stream has a callback set, the
    /// caller might need to manage extra locking.
    pub fn SDL_PutAudioStreamData(
        stream: *mut SDL_AudioStream,
        buf: *const c_void,
        len: c_int,
    ) -> bool;
}
