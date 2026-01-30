use crate::{SDL_BlendMode, SDL_Renderer};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set the blend mode used for drawing operations (Fill and Line).
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `blend_mode` - the [`SDL_BlendMode`] to use for blending.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// If the blend mode is not supported, the closest supported mode is chosen.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetRenderDrawBlendMode(
        renderer: *mut SDL_Renderer,
        blend_mode: SDL_BlendMode,
    ) -> bool;
}
