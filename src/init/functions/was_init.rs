use crate::SDL_InitFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_Init;

unsafe extern "C" {
    /// Get a mask of the specified subsystems which are currently initialized.
    ///
    /// # Parameters
    ///  * `flags` - any of the flags used by [`SDL_Init`]; see [`SDL_Init`] for details.
    ///
    /// # Return Value
    /// Returns a mask of all initialized subsystems if `flags` is 0, otherwise it returns the
    /// initialization status of the specified subsystems.
    pub fn SDL_WasInit(flags: SDL_InitFlags) -> SDL_InitFlags;
}
