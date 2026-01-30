use crate::{SDL_Renderer, SDL_Window};
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Create a 2D rendering context for a window.
    ///
    /// # Parameters
    ///  * `window` - the window where rendering is displayed.
    ///  * `name` - the name of the rendering driver to initialize, or [`null`] to let SDL choose
    ///             one.
    ///
    /// # Return Value
    /// Returns a valid rendering context or [`null_mut`] if there was an error; call
    /// [`SDL_GetError`] for more information.
    ///
    /// # Remarks
    /// If you want a specific renderer, you can specify its name here. A list of available
    /// renderers can be obtained by calling [`SDL_GetRenderDriver`] multiple times, with indices
    /// from 0 to `SDL_GetNumRenderDrivers() - 1`. If you don't need a specific renderer, specify
    /// [`null`] and SDL will attempt to choose the best option for you, based on what is available
    /// on the user's system.
    ///
    /// If `name` is a comma-separated list, SDL will try each name, in the order listed, until one
    /// succeeds or all of them fail.
    ///
    /// By default the rendering size matches the window size in pixels, but you can call
    /// [`SDL_SetRenderLogicalPresentation`] to change the content size and scaling options.
    pub fn SDL_CreateRenderer(window: *mut SDL_Window, name: *const c_char) -> *mut SDL_Renderer;
}
