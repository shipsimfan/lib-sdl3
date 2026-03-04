use crate::SDL_Surface;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Release a surface after directly accessing the pixels.
    ///
    /// # Parameters
    ///  * `surface` - the [`SDL_Surface`] structure to be unlocked.
    ///
    /// # Thread Safety
    /// This function is not thread safe. The locking referred to by this function is making the
    /// pixels available for direct access, not thread-safe locking.
    pub fn SDL_UnlockSurface(surface: *mut SDL_Surface);
}
