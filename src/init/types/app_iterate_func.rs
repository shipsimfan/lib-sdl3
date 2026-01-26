use crate::SDL_AppResult;
use std::ffi::c_void;

/// Function pointer typedef for [`SDL_AppIterate`].
///
/// # Parameters
///  * `appstate` - an optional pointer, provided by the app in [`SDL_AppInit`].
///
/// # Return Value
/// Returns [`SDL_AppResult::Failure`] to terminate with an error, [`SDL_AppResult::Success`] to
/// terminate with success, [`SDL_AppResult::Continue`] to continue.
///
/// # Remarks
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind the scenes for
/// apps using the optional main callbacks. Apps that want to use this should just implement
/// [`SDL_AppIterate`] directly.
#[allow(non_camel_case_types)]
pub type SDL_AppIterate_func = unsafe extern "C" fn(appstate: *mut c_void) -> SDL_AppResult;
