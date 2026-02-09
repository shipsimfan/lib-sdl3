use crate::SDL_Window;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_GetError};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Request that the size of a window's client area be set.
    ///
    /// # Parameters
    ///  * `window` - the window to change.
    ///  * `w` - the width of the window, must be > 0.
    ///  * `h` - the height of the window, must be > 0.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// If the window is in a fullscreen or maximized state, this request has no effect.
    ///
    /// To change the exclusive fullscreen mode of a window, use [`SDL_SetWindowFullscreenMode`].
    ///
    /// On some windowing systems, this request is asynchronous and the new window size may not
    /// have have been applied immediately upon the return of this function. If an immediate change
    /// is required, call [`SDL_SyncWindow`] to block until the changes have taken effect.
    ///
    /// When the window size changes, an [`SDL_EventType::WindowResized`] event will be emitted
    /// with the new window dimensions. Note that the new dimensions may not match the exact size
    /// requested, as some windowing systems can restrict the window size in certain scenarios
    /// (e.g. constraining the size of the content area to remain within the usable desktop
    /// bounds). Additionally, as this is just a request, it can be denied by the windowing system.
    pub fn SDL_SetWindowSize(window: *mut SDL_Window, w: c_int, h: c_int) -> bool;
}
