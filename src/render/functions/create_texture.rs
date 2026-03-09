use crate::{SDL_PixelFormat, SDL_Renderer, SDL_Texture, SDL_TextureAccess};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Create a texture for a rendering context.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `format` - one of the enumerated values in [`SDL_PixelFormat`].
    ///  * `access` - one of the enumerated values in [`SDL_TextureAccess`].
    ///  * `w` - the width of the texture in pixels.
    ///  * `h` - the height of the texture in pixels.
    ///
    /// # Return Value
    /// Returns the created texture or [`null_mut`] on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The contents of a texture when first created are not defined.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_CreateTexture(
        renderer: *mut SDL_Renderer,
        format: SDL_PixelFormat,
        access: SDL_TextureAccess,
        w: c_int,
        h: c_int,
    ) -> *mut SDL_Texture;
}
