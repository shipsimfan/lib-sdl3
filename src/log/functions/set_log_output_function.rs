use crate::SDL_LogOutputFunction;
use std::ffi::c_void;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Replace the default log output function with one of your own.
    ///
    /// # Parameters
    ///  * `callback` - an [`SDL_LogOutputFunction`] to call instead of the default.
    ///  * `userdata` - a pointer that is passed to `callback`.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_SetLogOutputFunction(callback: SDL_LogOutputFunction, userdata: *mut c_void);
}
