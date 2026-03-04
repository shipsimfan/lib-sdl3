use crate::{SDL_ScaleMode, SDL_Texture};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set the scale mode used for texture scale operations.
    ///
    /// # Parameters
    ///  * `texture` - the texture to update.
    ///  * `scale_mode` - the [`SDL_ScaleMode`] to use for texture scaling.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The default texture scale mode is [`SDL_ScaleMode::Linear`].
    ///
    /// If the scale mode is not supported, the closest supported mode is chosen.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetTextureScaleMode(texture: *mut SDL_Texture, scale_mode: SDL_ScaleMode) -> bool;
}
