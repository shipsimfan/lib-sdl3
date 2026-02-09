use crate::{SDL_Window, SDL_WindowID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_WindowEvent};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the numeric ID of a window.
    ///
    /// # Parameters
    ///  * `window` - the window to query.
    ///
    /// # Return Value
    /// Returns the ID of the window on success or 0 on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The numeric ID is what [`SDL_WindowEvent`] references, and is necessary to map these events
    /// to specific [`SDL_Window`] objects.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_GetWindowID(window: *mut SDL_Window) -> SDL_WindowID;
}
