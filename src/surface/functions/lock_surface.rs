use crate::SDL_Surface;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_UnlockSurface};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set up a surface for directly accessing the pixels.
    ///
    /// # Parameters
    ///  * `surface` - the [`SDL_Surface`] structure to be locked.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// Between calls to [`SDL_LockSurface`] / [`SDL_UnlockSurface`], you can write to and read
    /// from `surface.pixels`, using the pixel format stored in `surface.format`. Once you are done
    /// accessing the surface, you should use [`SDL_UnlockSurface`] to release it.
    ///
    /// Not all surfaces require locking. If `SDL_MUSTLOCK(surface)` evaluates to 0, then you can
    /// read and write to the surface at any time, and the pixel format of the surface will not
    /// change.
    ///
    /// # Thread Safety
    /// This function can be called on different threads with different surfaces. The locking
    /// referred to by this function is making the pixels available for direct access, not
    /// thread-safe locking.
    pub fn SDL_LockSurface(surface: *mut SDL_Surface) -> bool;
}
