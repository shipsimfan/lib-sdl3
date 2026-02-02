use crate::{SDL_FRect, SDL_Renderer};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Fill some number of rectangles on the current rendering target with the drawing color at
    /// subpixel precision.
    ///
    /// # Parameters
    ///  * `renderer` - the renderer which should fill multiple rectangles.
    ///  * `rects` - a pointer to an array of destination rectangles.
    ///  * `count` - the number of rectangles.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_RenderFillRects(
        renderer: *mut SDL_Renderer,
        rects: *const SDL_FRect,
        count: c_int,
    ) -> bool;
}
