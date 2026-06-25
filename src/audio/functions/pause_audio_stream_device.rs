use crate::SDL_AudioStream;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_OpenAudioDeviceStream};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Use this function to pause audio playback on the audio device associated with an audio
    /// stream.
    ///
    /// # Parameters
    ///  * `stream` - the audio stream associated with the audio device to pause.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This function pauses audio processing for a given device. Any bound audio streams will not
    /// progress, and no audio will be generated. Pausing one device does not prevent other
    /// unpaused devices from running.
    ///
    /// Pausing a device can be useful to halt all audio without unbinding all the audio streams.
    /// This might be useful while a game is paused, or a level is loading, etc.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_PauseAudioStreamDevice(stream: *mut SDL_AudioStream) -> bool;
}
