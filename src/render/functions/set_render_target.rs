use crate::{SDL_Renderer, SDL_Texture};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set a texture as the current rendering target.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `texture` - the targeted texture, which must be created with the
    ///                [`SDL_TextureAccess::Target`] flag, or [`null_mut`] to render to the window
    ///                instead of a texture.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The default render target is the window for which the renderer was created. To stop
    /// rendering to a texture and render to the window again, call this function with a
    /// [`null_mut`] texture.
    ///
    /// Viewport, cliprect, scale, and logical presentation are unique to each render target. Get
    /// and set functions for these states apply to the current render target set by this function,
    /// and those states persist on each target when the current render target changes.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetRenderTarget(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture) -> bool;
}
