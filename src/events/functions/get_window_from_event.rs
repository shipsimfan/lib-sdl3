use crate::{SDL_Event, SDL_Window};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get window associated with an event.
    ///
    /// # Parameters
    ///  * `event` - an event containing a `window_id`.
    ///
    /// # Return Value
    /// Returns the associated window on success or [`null_mut`] if there is none.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetWindowFromEvent(event: *const SDL_Event) -> *mut SDL_Window;
}
