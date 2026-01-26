use crate::{SDL_AppResult, SDL_Event};
use std::ffi::c_void;

/// Function pointer typedef for [`SDL_AppEvent`].
///
/// # Parameters
///  * `appstate` - an optional pointer, provided by the app in [`SDL_AppInit`].
///  * `event` - the new event for the app to examine.
///
/// # Return Value
/// Returns [`SDL_AppResult::Failure`] to terminate with an error, [`SDL_AppResult::Success`] to
/// terminate with success, [`SDL_AppResult::Continue`] to continue.
///
/// # Remarks
/// These are used by [`SDL_EnterAppMainCallbacks`]. This mechanism operates behind the scenes for
/// apps using the optional main callbacks. Apps that want to use this should just implement
/// [`SDL_AppEvent`] directly.
#[allow(non_camel_case_types)]
pub type SDL_AppEvent_func =
    unsafe extern "C" fn(appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult;
