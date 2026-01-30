use crate::{SDL_Renderer, SDL_Window, SDL_WindowFlags};
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_CreateWindow, SDL_GetError};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Create a window and default renderer.
    ///
    /// # Parameters
    ///  * `title` - the title of the window, in UTF-8 encoding.
    ///  * `width` - the width of the window.
    ///  * `height` - the height of the window.
    ///  * `window_flags` - the flags used to create the window (see [`SDL_CreateWindow`]).
    ///  * `window` - a pointer filled with the window, or [`null_mut`] on error.
    ///  * `renderer` - a pointer filled with the renderer, or [`null_mut`] on error.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_CreateWindowAndRenderer(
        title: *const c_char,
        width: c_int,
        height: c_int,
        window_flags: SDL_WindowFlags,
        window: *mut *mut SDL_Window,
        renderer: *mut *mut SDL_Renderer,
    ) -> bool;
}
