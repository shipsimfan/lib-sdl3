use crate::SDL_InitFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_Init};

#[link(name = "SDL3")]
extern "C" {
    /// Compatibility function to initialize the SDL library.
    ///
    /// # Parameters
    ///  * `flags` - any of the flags used by [`SDL_Init`]; see [`SDL_Init`] for details.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This function and [`SDL_Init`] are interchangeable.
    pub fn SDL_InitSubSystem(flags: SDL_InitFlags) -> bool;
}
