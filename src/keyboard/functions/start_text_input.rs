use crate::SDL_Window;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_GetError, SDL_StopTextInput};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Start accepting Unicode text input events in a window.
    ///
    /// # Parameters
    ///  * `window` - the window to enable text input.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This function will enable text input ([`SDL_EventType::TextInput`] and
    /// [`SDL_EventType::TextEditing`] events) in the specified window. Please use this function
    /// paired with [`SDL_StopTextInput`].
    ///
    /// Text input events are not received by default.
    ///
    /// On some platforms using this function shows the screen keyboard and/or activates an IME,
    /// which can prevent some key press events from being passed through.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_StartTextInput(window: *mut SDL_Window) -> bool;
}
