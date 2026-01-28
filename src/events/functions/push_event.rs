use crate::SDL_Event;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_PeepEvents, SDL_RegisterEvents};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Add an event to the event queue.
    ///
    /// # Parameters
    ///  * `event` - the [`SDL_Event`] to be added to the queue.
    ///
    /// # Return Value
    /// Returns `true` on success, `false` if the event was filtered or on failure; call
    /// [`SDL_GetError`] for more information. A common reason for error is the event queue being
    /// full.
    ///
    /// # Remarks
    /// The event queue can actually be used as a two way communication channel. Not only can
    /// events be read from the queue, but the user can also push their own events onto it. event
    /// is a pointer to the event structure you wish to push onto the queue. The event is copied
    /// into the queue, and the caller may dispose of the memory pointed to after [`SDL_PushEvent`]
    /// returns.
    ///
    /// Note: Pushing device input events onto the queue doesn't modify the state of the device
    /// within SDL.
    ///
    /// Note: Events pushed onto the queue with [`SDL_PushEvent`] get passed through the event
    /// filter but events added with [`SDL_PeepEvents`] do not.
    ///
    /// For pushing application-specific events, please use [`SDL_RegisterEvents`] to get an event
    /// type that does not conflict with other code that also wants its own custom event types.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_PushEvent(event: *mut SDL_Event) -> bool;
}
