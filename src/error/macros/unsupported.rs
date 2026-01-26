// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_SetError;

/// A macro to standardize error reporting on unsupported operations.
///
/// # Remarks
/// This simply calls [`SDL_SetError`] with a standardized error string, for convenience,
/// consistency, and clarity.
///
/// # Thread Safety
/// It is safe to call this macro from any thread.
#[macro_export]
macro_rules! SDL_Unsupported {
    () => {
        $crate::SDL_SetError(c"That operation is not supported".as_ptr())
    };
}
