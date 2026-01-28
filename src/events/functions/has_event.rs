// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_HasEvents};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Check for the existence of a certain event type in the event queue.
    ///
    /// # Parameters
    ///  * `r#type` - the type of event to be queried; see [`SDL_EventType`] for details.
    ///
    /// # Return Value
    /// Returns `true` if events matching type are present, or `false` if events matching type are
    /// not present.
    ///
    /// # Remarks
    /// If you need to check for a range of event types, use [`SDL_HasEvents`] instead.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_HasEvent(r#type: u32) -> bool;
}
