use crate::SDL_EventFilter;
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_SetEventFilter;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Run a specific filter function on the current event queue, removing any events for which
    /// the filter returns `false`.
    ///
    /// # Parameters
    ///  * `filter` - the [`SDL_EventFilter`] function to call when an event happens.
    ///  * `userdata` - a pointer that is passed to `filter`.
    ///
    /// # Remarks
    /// See [`SDL_SetEventFilter`] for more information. Unlike [`SDL_SetEventFilter`], this
    /// function does not change the filter permanently, it only uses the supplied filter until
    /// this function returns.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *mut c_void);
}
