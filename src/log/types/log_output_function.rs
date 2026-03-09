use crate::SDL_LogPriority;
use std::ffi::{c_char, c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_SetLogOutputFunction;

/// The prototype for the log output callback function.
///
/// # Parameters
///  * `userdata` - what was passed as `userdata` to [`SDL_SetLogOutputFunction`].
///  * `category` - the category of the message.
///  * `priority` - the priority of the message.
///  * `message` - the message being output.
///
/// # Remarks
/// This function is called by SDL when there is new text to be logged. A mutex is held so that
/// this function is never called by more than one thread at once.
#[allow(non_camel_case_types)]
pub type SDL_LogOutputFunction = unsafe extern "C" fn(
    userdata: *mut c_void,
    category: c_int,
    priority: SDL_LogPriority,
    message: *const c_char,
);
