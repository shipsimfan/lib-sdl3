use crate::SDL_Event;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PumpEvents;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Wait until the specified timeout (in milliseconds) for the next available event.
    ///
    /// # Parameters
    ///  * `event` - the [`SDL_Event`] structure to be filled in with the next event from the
    ///              queue, or [`null_mut`].
    ///  * `timeout_ms` - the maximum number of milliseconds to wait for the next available event.
    ///
    /// # Return Value
    /// Returns `true` if this got an event or `false` if the timeout elapsed without any events
    /// available.
    ///
    /// # Remarks
    /// If `event` is not [`null_mut`], the next event is removed from the queue and stored in the
    /// [`SDL_Event`] structure pointed to by `event`.
    ///
    /// As this function may implicitly call [`SDL_PumpEvents`], you can only call this function in
    /// the thread that initialized the video subsystem.
    ///
    /// The timeout is not guaranteed, the actual wait time could be longer due to system
    /// scheduling.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_WaitEventTimeout(event: *mut SDL_Event, timeout_ms: i32) -> bool;
}
