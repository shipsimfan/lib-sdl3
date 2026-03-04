use crate::{SDL_PixelFormat, SDL_PixelFormatDetails};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Create an [`SDL_PixelFormatDetails`] structure corresponding to a pixel format.
    ///
    /// # Parameters
    ///  * `format` - one of the [`SDL_PixelFormat`] values.
    ///
    /// # Return Value
    /// Returns a pointer to a [`SDL_PixelFormatDetails`] structure or [`null`] on failure; call
    /// [`SDL_GetError`] for more information.
    ///
    /// # Remarks
    /// Returned structure may come from a shared global cache (i.e. not newly allocated), and
    /// hence should not be modified, especially the palette. Weird errors such as Blit combination
    /// not supported may occur.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetPixelFormatDetails(format: SDL_PixelFormat) -> *const SDL_PixelFormatDetails;
}
