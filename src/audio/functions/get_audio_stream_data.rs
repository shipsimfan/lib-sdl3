use crate::SDL_AudioStream;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_PutAudioStreamData, SDL_SetAudioStreamFormat};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get converted/resampled data from the stream.
    ///
    /// # Parameters
    ///  * `stream` - the stream the audio is being requested from.
    ///  * `buf` - a buffer to fill with audio data.
    ///  * `len` - the maximum number of bytes to fill.
    ///
    /// # Return Value
    /// Returns the number of bytes read from the stream or -1 on failure; call [`SDL_GetError`]
    /// for more information.
    ///
    /// # Remarks
    /// The input/output data format/channels/samplerate is specified when creating the stream, and
    /// can be changed after creation by calling [`SDL_SetAudioStreamFormat`].
    ///
    /// Note that any conversion and resampling necessary is done during this call, and
    /// [`SDL_PutAudioStreamData`] simply queues unconverted data for later. This is different than
    /// SDL2, where that work was done while inputting new data to the stream and requesting the
    /// output just copied the converted data.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread, but if the stream has a callback set, the
    /// caller might need to manage extra locking.
    pub fn SDL_GetAudioStreamData(
        stream: *mut SDL_AudioStream,
        buf: *mut c_void,
        len: c_int,
    ) -> c_int;
}
