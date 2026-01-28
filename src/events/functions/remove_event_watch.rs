use crate::SDL_EventFilter;
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_AddEventWatch;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Remove an event watch callback added with [`SDL_AddEventWatch`].
    ///
    /// # Parameters
    ///  * `filter` - the function originally passed to [`SDL_AddEventWatch`].
    ///  * `userdata` - the pointer originally passed to [`SDL_AddEventWatch`].
    ///
    /// # Remarks
    /// This function takes the same input as [`SDL_AddEventWatch`] to identify and delete the
    /// corresponding callback.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_RemoveEventWatch(filter: SDL_EventFilter, userdata: *mut c_void);
}
