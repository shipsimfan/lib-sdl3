use std::ffi::c_char;

#[link(name = "SDL3")]
extern "C" {
    /// Set the SDL error message for the current thread.
    ///
    /// # Parameters
    ///  * `fmt` - a `printf`-style message format string.
    ///  * `...` - additional parameters matching % tokens in the `fmt` string, if any.
    ///
    /// # Return Value
    /// Returns `false`.
    ///
    /// # Remarks
    /// Calling this function will replace any previous error message that was set.
    ///
    /// This function always returns `false`, since SDL frequently uses `false` to signify a
    /// failing result, leading to this idiom:
    /// ```no_run
    /// if error_code {
    ///    return SDL_SetError("This operation has failed: %d", error_code);
    /// }
    /// ```
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_SetError(fmt: *const c_char, ...) -> bool;
}
