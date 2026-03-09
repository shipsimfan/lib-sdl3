use crate::{SDL_Rect, SDL_Texture};
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Update the given texture rectangle with new pixel data.
    ///
    /// # Parameters
    ///  * `texture` - the texture to update.
    ///  * `rect` - an [`SDL_Rect`] structure representing the area to update, or [`null`] to
    ///             update the entire texture.
    ///  * `pixels` - the raw pixel data in the format of the texture.
    ///  * `pitch` - the number of bytes in a row of pixel data, including padding between lines.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The pixel data must be in the pixel format of the texture, which can be queried using the
    /// [`SDL_PROP_TEXTURE_FORMAT_NUMBER`] property.
    ///
    /// This is a fairly slow function, intended for use with static textures that do not change
    /// often.
    ///
    /// If the texture is intended to be updated often, it is preferred to create the texture as
    /// streaming and use the locking functions referenced below. While this function will work
    /// with streaming textures, for optimization reasons you may not get the pixels back if you
    /// lock the texture afterward.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_UpdateTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *const c_void,
        pitch: c_int,
    ) -> bool;
}
