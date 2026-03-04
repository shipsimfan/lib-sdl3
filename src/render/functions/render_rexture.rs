use crate::{SDL_FRect, SDL_Renderer, SDL_Texture};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Copy a portion of the texture to the current rendering target at subpixel precision.
    ///
    /// # Parameters
    ///  * `renderer` - the renderer which should copy parts of a texture.
    ///  * `texture` - the source texture.
    ///  * `src_rect` - a pointer to the source rectangle, or [`null`] for the entire texture.
    ///  * `dst_rect` - a pointer to the destination rectangle, or [`null`] for the entire
    ///                 rendering target.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_RenderTexture(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        src_rect: *const SDL_FRect,
        dst_rect: *const SDL_FRect,
    ) -> bool;
}
