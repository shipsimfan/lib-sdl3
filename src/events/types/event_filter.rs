use crate::SDL_Event;
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_AddEventWatch, SDL_SetEventFilter};

/// A function pointer used for callbacks that watch the event queue.
///
/// # Parameters
///  * `userdata` - what was passed as userdata to [`SDL_SetEventFilter`] or [`SDL_AddEventWatch`],
///                 etc.
///  * `event` - the event that triggered the callback.
///
/// # Return Value
/// Returns `true` to permit event to be added to the queue, and `false` to disallow it. When used
/// with [`SDL_AddEventWatch`], the return value is ignored.
///
/// # Thread Safety
/// SDL may call this callback at any time from any thread; the application is responsible for
/// locking resources the callback touches that need to be protected.
#[allow(non_camel_case_types)]
pub type SDL_EventFilter = extern "C" fn(userdata: *mut c_void, event: *mut SDL_Event) -> bool;
