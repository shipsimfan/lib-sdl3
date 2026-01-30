use crate::{SDL_Rect, SDL_Renderer};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set the clip rectangle for rendering on the specified target.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `rect` - an [`SDL_Rect`] structure representing the clip area, relative to the viewport,
    ///             or [`null`] to disable clipping.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// Each render target has its own clip rectangle. This function sets the cliprect for the
    /// current render target.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetRenderClipRect(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> bool;
}
