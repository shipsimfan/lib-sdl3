use crate::SDL_Renderer;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the output size in pixels of a rendering context.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `w` - a pointer filled in with the width in pixels.
    ///  * `h` - a pointer filled in with the height in pixels.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    pub fn SDL_GetRenderOutputSize(
        renderer: *mut SDL_Renderer,
        w: *mut c_int,
        h: *mut c_int,
    ) -> bool;
}
