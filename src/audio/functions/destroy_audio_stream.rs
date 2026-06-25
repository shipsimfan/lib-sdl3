use crate::SDL_AudioStream;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_OpenAudioDeviceStream;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Free an audio stream.
    ///
    /// # Parameters
    ///  * `stream` - the audio stream to destroy.
    ///
    /// # Remarks
    /// This will release all allocated data, including any audio that is still queued. You do not
    /// need to manually clear the stream first.
    ///
    /// If this stream was bound to an audio device, it is unbound during this call. If this stream
    /// was created with [`SDL_OpenAudioDeviceStream`], the audio device that was opened alongside
    /// this stream's creation will be closed, too.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_DestroyAudioStream(stream: *mut SDL_AudioStream);
}
