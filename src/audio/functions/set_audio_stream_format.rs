use crate::{SDL_AudioSpec, SDL_AudioStream};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SDL_GetAudioStreamAvailable, SDL_GetAudioStreamData, SDL_GetError, SDL_PutAudioStreamData,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Change the input and output formats of an audio stream.
    ///
    /// # Parameters
    ///  * `stream` - the stream the format is being changed.
    ///  * `src_spec` - the new format of the audio input; if [`null`], it is not changed.
    ///  * `dst_spec` - the new format of the audio output; if [`null`], it is not changed.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// Future calls to and [`SDL_GetAudioStreamAvailable`] and [`SDL_GetAudioStreamData`] will
    /// reflect the new format, and future calls to [`SDL_PutAudioStreamData`] must provide data in
    /// the new input formats.
    ///
    /// Data that was previously queued in the stream will still be operated on in the format that
    /// was current when it was added, which is to say you can put the end of a sound file in one
    /// format to a stream, change formats for the next sound file, and start putting that new data
    /// while the previous sound file is still queued, and everything will still play back
    /// correctly.
    ///
    /// If a stream is bound to a device, then the format of the side of the stream bound to a
    /// device cannot be changed (`src_spec` for recording devices, `dst_spec` for playback
    /// devices). Attempts to make a change to this side will be ignored, but this will not report
    /// an error. The other side's format can be changed.
    ///
    /// `src_spec` and `dst_spec` may each be [`null`]; a [`null`] spec signals not to change the
    /// current format for that side of the stream.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread, as it holds a stream-specific mutex while
    /// running.
    pub fn SDL_SetAudioStreamFormat(
        stream: *mut SDL_AudioStream,
        src_spec: *const SDL_AudioSpec,
        dst_spec: *const SDL_AudioSpec,
    ) -> bool;
}
