use crate::SDL_Renderer;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_SetRenderDrawColor};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Clear the current rendering target with the drawing color.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This function clears the entire rendering target, ignoring the viewport and the clip
    /// rectangle. Note, that clearing will also set/fill all pixels of the rendering target to
    /// current renderer draw color, so make sure to invoke [`SDL_SetRenderDrawColor`] when needed.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> bool;
}
