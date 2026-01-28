use std::ffi::c_int;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Allocate a set of user-defined events, and return the beginning event number for that set of events.
    ///
    /// # Parameters
    ///  * `numevents` - the number of events to be allocated.
    ///
    /// # Return Value
    /// Returns the beginning event number, or 0 if `numevents` is invalid or if there are not
    /// enough user-defined events left.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_RegisterEvents(numevents: c_int) -> u32;
}
