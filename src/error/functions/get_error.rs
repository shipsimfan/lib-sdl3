use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_ClearError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Retrieve a message about the last error that occurred on the current thread.
    ///
    /// # Return Value
    /// Returns a message with information about the specific error that occurred, or an empty
    /// string if there hasn't been an error message set since the last call to [`SDL_ClearError`].
    ///
    /// # Remarks
    /// It is possible for multiple errors to occur before calling [`SDL_GetError`]. Only the last
    /// error is returned.
    ///
    /// The message is only applicable when an SDL function has signaled an error. You must check
    /// the return values of SDL function calls to determine when to appropriately call
    /// [`SDL_GetError`]. You should not use the results of [`SDL_GetError`] to decide if an error
    /// has occurred! Sometimes SDL will set an error string even when reporting success.
    ///
    /// SDL will not clear the error string for successful API calls. You must check return values
    /// for failure cases before you can assume the error string applies.
    ///
    /// Error strings are set per-thread, so an error set in a different thread will not interfere
    /// with the current thread's operation.
    ///
    /// The returned value is a thread-local string which will remain valid until the current
    /// thread's error string is changed. The caller should make a copy if the value is needed
    /// after the next SDL API call.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetError() -> *const c_char;
}
