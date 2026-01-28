use crate::SDL_Event;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Generate an English description of an event.
    ///
    /// # Parameters
    ///  * `event` - an event to describe. May be [`null`].
    ///  * `buf` - the buffer to fill with the description string. May be [`null_mut`].
    ///  * `buflen` - the maximum bytes that can be written to `buf`.
    ///
    /// # Return Value
    /// Returns number of bytes needed for the full string, not counting the null-terminator byte.
    ///
    /// # Remarks
    /// This will fill `buf` with a null-terminated string that might look something like this:
    ///
    /// "SDL_EVENT_MOUSE_MOTION (timestamp=1140256324 windowid=2 which=0 state=0 x=492.99 y=139.09
    /// xrel=52 yrel=6)"
    ///
    /// The exact format of the string is not guaranteed; it is intended for logging purposes, to
    /// be read by a human, and not parsed by a computer.
    ///
    /// The returned value follows the same rules as [`SDL_snprintf`]: `buf` will always be
    /// NULL-terminated (unless `buflen` is zero), and will be truncated if `buflen` is too small.
    /// The return code is the number of bytes needed for the complete string, not counting the
    /// NULL-terminator, whether the string was truncated or not. Unlike [`SDL_snprintf`], though,
    /// this function never returns -1.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetEventDescription(
        event: *const SDL_Event,
        buf: *mut c_char,
        buflen: c_int,
    ) -> c_int;
}
