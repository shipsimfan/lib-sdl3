use crate::SDL_Texture;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set an additional alpha value multiplied into render copy operations.
    ///
    /// # Parameters
    ///  * `texture` - the texture to update.
    ///  * `alpha` - the alpha value multiplied into copy operations.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// When this texture is rendered, during the copy operation the source alpha value is
    /// modulated by this alpha value according to the following formula:
    /// ```no_run
    /// src_a = src_a * (alpha / 255)
    /// ```     
    ///
    /// Alpha modulation is not always supported by the renderer; it will return `false` if alpha
    /// modulation is not supported.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetTextureAlphaMod(texture: *mut SDL_Texture, alpha: u8) -> bool;
}
