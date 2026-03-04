use crate::SDL_Surface;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Free a surface.
    ///
    /// # Parameters
    ///  * `surface` - the [`SDL_Surface`] to free.
    ///
    /// # Remarks
    /// It is safe to pass [`null_mut`] to this function.
    ///
    /// # Thread Safety
    /// No other thread should be using the surface when it is freed.
    pub fn SDL_DestroySurface(surface: *mut SDL_Surface);
}
