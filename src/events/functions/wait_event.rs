use crate::SDL_Event;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_PumpEvents};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Wait indefinitely for the next available event.
    ///
    /// # Parameters
    ///  * `event` - the [`SDL_Event`] structure to be filled in with the next event from the
    ///              queue, or [`null_mut`].
    ///
    /// # Return Value
    /// Returns true on success or false if there was an error while waiting for events; call
    /// [`SDL_GetError`] for more information.
    ///
    /// # Remarks
    /// If `event` is not [`null_mut`], the next event is removed from the queue and stored in the
    /// [`SDL_Event`] structure pointed to by `event`.
    ///
    /// As this function may implicitly call [`SDL_PumpEvents`], you can only call this function in
    /// the thread that initialized the video subsystem.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_WaitEvent(event: *mut SDL_Event) -> bool;
}
