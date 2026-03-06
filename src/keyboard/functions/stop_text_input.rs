use crate::SDL_Window;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_StartTextInput};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Stop receiving any text input events in a window.
    ///
    /// # Parameters
    ///  * `window` - the window to disable text input.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// If [`SDL_StartTextInput`] showed the screen keyboard, this function will hide it.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_StopTextInput(window: *mut SDL_Window) -> bool;
}
