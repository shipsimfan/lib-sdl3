use std::ffi::c_void;

/// Callback run on the main thread.
///
/// # Parameters
///  * `userdata` - an app-controlled pointer that is passed to the callback.
#[allow(non_camel_case_types)]
pub type SDL_MainThreadCallback = unsafe extern "C" fn(userdata: *mut c_void);
