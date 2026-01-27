// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_SetError;

/// A macro to standardize error reporting on unsupported operations.
///
/// # Remarks
/// This simply calls [`SDL_SetError`] with a standardized error string, for convenience,
/// consistency, and clarity.
///
/// A common usage pattern inside SDL is this:
/// ```no_run
/// fn my_function(str: *const std::ffi::c_char) -> bool {
///    if str == std::ptr::null() {
///         return SDL_InvalidParamError!(c"str".as_ptr());  // returns false.
///     }
///     do_something(str);
///     return true;
/// }
/// ```
///
/// # Thread Safety
/// It is safe to call this macro from any thread.
#[macro_export]
macro_rules! SDL_InvalidParamError {
    ($param: expr) => {
        $crate::SDL_SetError(c"Parameter '%s' is invalid".as_ptr(), $param)
    };
}
