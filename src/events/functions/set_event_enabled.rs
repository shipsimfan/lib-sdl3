// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_EventType;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set the state of processing events by type.
    ///
    /// # Parameters
    ///  * `r#type` - the type of event; see [`SDL_EventType`] for details.
    ///  * `enabled` - whether to process the event or not.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_SetEventEnabled(r#type: u32, enabled: bool);
}
