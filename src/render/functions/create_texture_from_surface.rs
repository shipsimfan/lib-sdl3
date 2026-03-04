use crate::{SDL_Renderer, SDL_Surface, SDL_Texture};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Create a texture from an existing surface.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `surface` - the [`SDL_Surface`] structure containing pixel data used to fill the
    ///                texture.
    ///
    /// # Return Value
    /// Returns the created texture or [`null_mut`] on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The surface is not modified or freed by this function.
    ///
    /// The [`SDL_TextureAccess`] hint for the created texture is [`SDL_TEXTUREACCESS_STATIC`].
    ///
    /// The pixel format of the created texture may be different from the pixel format of the
    /// surface, and can be queried using the [`SDL_PROP_TEXTURE_FORMAT_NUMBER`] property.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_CreateTextureFromSurface(
        renderer: *mut SDL_Renderer,
        surface: *mut SDL_Surface,
    ) -> *mut SDL_Texture;
}
