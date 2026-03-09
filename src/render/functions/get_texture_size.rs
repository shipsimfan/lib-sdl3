use crate::SDL_Texture;
use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the size of a texture, as floating point values.
    ///
    /// # Parameters
    ///  * `texture` - the texture to query.
    ///  * `w` - a pointer filled in with the width of the texture in pixels. This argument can be
    ///          [`null_mut`] if you don't need this information.
    ///  * `h` - a pointer filled in with the height of the texture in pixels. This argument can be
    ///          [`null_mut`] if you don't need this information.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_GetTextureSize(texture: *mut SDL_Texture, w: *mut c_float, h: *mut c_float) -> bool;
}
