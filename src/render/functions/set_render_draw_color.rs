use crate::SDL_Renderer;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_RenderClear};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set the color used for drawing operations.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `r` - the red value used to draw on the rendering target.
    ///  * `g` - the green value used to draw on the rendering target.
    ///  * `b` - the blue value used to draw on the rendering target.
    ///  * `a` - the alpha value used to draw on the rendering target; usually
    ///          [`SDL_ALPHA_OPAQUE`]. Use [`SDL_SetRenderDrawBlendMode`] to specify how the alpha
    ///          channel is used.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// Set the color for drawing or filling rectangles, lines, and points, and for
    /// [`SDL_RenderClear`].
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetRenderDrawColor(renderer: *mut SDL_Renderer, r: u8, g: u8, b: u8, a: u8) -> bool;
}
