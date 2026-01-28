use crate::SDL_EventFilter;
use std::ffi::c_void;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Query the current event filter.
    ///
    /// # Parameters
    ///  * `filter` - the current callback function will be stored here.
    ///  * `userdata` - the pointer that is passed to the current event filter will be stored here.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` if there is no event filter set.
    ///
    /// # Remarks
    /// This function can be used to "chain" filters, by saving the existing filter before
    /// replacing it with a function that will call that saved filter.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetEventFilter(filter: *mut SDL_EventFilter, userdata: *mut *mut c_void) -> bool;
}
