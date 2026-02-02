use crate::{SDL_FRect, SDL_Renderer};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Fill a rectangle on the current rendering target with the drawing color at subpixel
    /// precision.
    ///
    /// # Parameters
    ///  * `renderer` - the renderer which should fill a rectangle.
    ///  * `rect` - a pointer to the destination rectangle, or [`null`] for the entire rendering
    ///             target.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_RenderFillRect(renderer: *mut SDL_Renderer, rect: *const SDL_FRect) -> bool;
}
