use std::ffi::{c_char, c_int, c_void};

/// Function pointer typedef for SDL_AppInit.
///
/// # Parameters
///  * `appstate` - a place where the app can optionally store a pointer for future use.
///  * `argc` - the standard ANSI C main's argc; number of elements in `argv`.
///  * `argv` - the standard ANSI C main's argv; array of command line arguments.
///
/// # Return Value
/// Returns [`SDL_AppResult::Failure`] to terminate with an error, [`SDL_AppResult::Success`] to
/// terminate with success, [`SDL_AppResult::Continue`] to continue.
///
/// # Remarks
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind the scenes for
/// apps using the optional main callbacks. Apps that want to use this should just implement
/// [`SDL_AppInit`] directly.
#[allow(non_camel_case_types)]
pub type SDL_AppInit_func =
    unsafe extern "C" fn(appstate: *mut *mut c_void, argc: c_int, argv: *mut *mut c_char);
