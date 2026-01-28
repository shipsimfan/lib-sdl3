use crate::SDL_InitFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_Init, SDL_Quit};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Shut down specific SDL subsystems.
    ///
    /// # Parameters
    ///  * `flags` - any of the flags used by [`SDL_Init`]; see [`SDL_Init`] for details.
    ///
    /// # Remarks
    /// You still need to call [`SDL_Quit`] even if you close all open subsystems with
    /// [`SDL_QuitSubSystem`].
    pub fn SDL_QuitSubSystem(flags: SDL_InitFlags);
}
