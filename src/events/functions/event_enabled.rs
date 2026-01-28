// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_EventType;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Query the state of processing events by type.
    ///
    /// # Parameters
    ///  * `r#type` - the type of event; see [`SDL_EventType`] for details.
    ///
    /// # Return Value
    /// Returns `true` if the event is being processed, `false` otherwise.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_EventEnabled(r#type: u32) -> bool;
}
