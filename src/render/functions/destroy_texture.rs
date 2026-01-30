use crate::SDL_Texture;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Destroy the specified texture.
    ///
    /// # Parameters
    ///  * `texture` - the texture to destroy.
    ///
    /// # Remarks
    /// Passing [`null_mut`] or an otherwise invalid texture will set the SDL error message to
    /// "Invalid texture".
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
}
