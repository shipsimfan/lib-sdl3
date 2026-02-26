use crate::SDL_Window;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the size of a window's client area.
    ///
    /// # Parameters
    ///  * `window` - the window to query the width and height from.
    ///  * `w` - a pointer filled in with the width of the window, may be [`null_mut`].
    ///  * `h` - a pointer filled in with the height of the window, may be [`null_mut`].
    ///
    /// # Return Value
    /// Returns true on success or false on failure; call [`SDL_GetError`] for more information.
    ///
    /// # Remarks
    /// The window pixel size may differ from its window coordinate size if the window is on a high
    /// pixel density display. Use [`SDL_GetWindowSizeInPixels`] or [`SDL_GetRenderOutputSize`] to
    /// get the real client area size in pixels.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_GetWindowSize(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int) -> bool;
}
