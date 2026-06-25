use crate::SDL_AudioStream;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_OpenAudioDeviceStream};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Use this function to unpause audio playback on the audio device associated with an audio
    /// stream.
    ///
    /// # Parameters
    ///  * `stream` - the audio stream associated with the audio device to resume.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This function unpauses audio processing for a given device that has previously been paused.
    /// Once unpaused, any bound audio streams will begin to progress again, and audio can be
    /// generated.
    ///
    /// [`SDL_OpenAudioDeviceStream`] opens audio devices in a paused state, so this function call
    /// is required for audio playback to begin on such devices.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_ResumeAudioStreamDevice(stream: *mut SDL_AudioStream) -> bool;
}
