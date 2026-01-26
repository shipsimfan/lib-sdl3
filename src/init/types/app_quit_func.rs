use crate::SDL_AppResult;
use std::ffi::c_void;

/// Function pointer typedef for [`SDL_AppQuit`].
///
/// # Parameters
///  * `appstate` - an optional pointer, provided by the app in [`SDL_AppInit`].
///  * `result` - the result code that terminated the app (success or failure).
///
/// # Remarks
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind the scenes for
/// apps using the optional main callbacks. Apps that want to use this should just implement
/// [`SDL_AppEvent`] directly.
#[allow(non_camel_case_types)]
pub type SDL_AppQuit_func = unsafe extern "C" fn(appstate: *mut c_void, result: SDL_AppResult);
