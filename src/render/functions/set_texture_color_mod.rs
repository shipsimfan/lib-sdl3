use crate::SDL_Texture;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set an additional color value multiplied into render copy operations.
    ///
    /// # Parameters
    ///  * `texture` - the texture to update.
    ///  * `r` - the red color value multiplied into copy operations.
    ///  * `g` - the green color value multiplied into copy operations.
    ///  * `b` - the blue color value multiplied into copy operations.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// When this texture is rendered, during the copy operation each source color channel is
    /// modulated by the appropriate color value according to the following formula:
    /// ```no_run
    /// src_c = src_c * (color / 255)
    /// ```
    ///
    /// Color modulation is not always supported by the renderer; it will return `false` if color
    /// modulation is not supported.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetTextureColorMod(texture: *mut SDL_Texture, r: u8, g: u8, b: u8) -> bool;
}
