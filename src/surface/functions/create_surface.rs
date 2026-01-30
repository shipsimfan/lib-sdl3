use crate::{SDL_PixelFormat, SDL_Surface};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Allocate a new surface with a specific pixel format.
    ///
    /// # Parameters
    ///  * `width` - the width of the surface.
    ///  * `height` - the height of the surface.
    ///  * `format` - the [`SDL_PixelFormat`] for the new surface's pixel format.
    ///
    /// # Return Value
    /// Returns the new [`SDL_Surface`] structure that is created or [`null_mut`] on failure; call
    /// [`SDL_GetError`] for more information.
    ///
    /// # Remarks
    /// The pixels of the new surface are initialized to zero.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_CreateSurface(
        width: c_int,
        height: c_int,
        format: SDL_PixelFormat,
    ) -> *mut SDL_Surface;
}
