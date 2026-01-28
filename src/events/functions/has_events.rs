// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_HasEvent};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Check for the existence of certain event types in the event queue.
    ///
    /// # Parameters
    ///  * `min_type` - the low end of event type to be queried, inclusive; see [`SDL_EventType`]
    ///                 for details.
    ///  * `max_type` - the high end of event type to be queried, inclusive; see [`SDL_EventType`]
    ///                 for details.
    ///
    /// # Return Value
    /// Returns `true` if events with `type >= min_type` and `<= max_type` are present, or `false`
    /// if not.
    ///
    /// # Remarks
    /// If you need to check for a single event type, use [`SDL_HasEvent`] instead.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_HasEvents(min_type: u32, max_type: u32) -> bool;
}
