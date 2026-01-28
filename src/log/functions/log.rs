use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_LogCategory, SDL_LogPriority};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Log a message with [`SDL_LogCategory::Application`] and [`SDL_LogPriority::Info`].
    ///
    /// # Parameters
    ///  * `fmt` - a `printf` style message format string.
    ///  * `...` - additional parameters matching % tokens in the `fmt` string, if any.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_Log(fmt: *const c_char, ...);
}
