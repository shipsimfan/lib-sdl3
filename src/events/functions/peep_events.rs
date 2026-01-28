use crate::{SDL_Event, SDL_EventAction};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_GetError, SDL_PumpEvents};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Check the event queue for messages and optionally return them.
    ///
    /// # Parameters
    ///  * `events` - destination buffer for the retrieved events, may be [`null_mut`] to leave the
    ///               events in the queue and return the number of events that would have been
    ///               stored.
    ///  * `numevents` - if `action` is [`SDL_EventAction::AddEvent`], the number of events to add
    ///                  back to the event queue; if `action` is [`SDL_EventAction::PeekEvent`] or
    ///                  [`SDL_EventAction::GetEvent`], the maximum number of events to retrieve.
    ///  * `action` - action to take; see Remarks for details.
    ///  * `min_type` - minimum value of the event type to be considered; [`SDL_EventType::Last`]
    ///                 is a safe choice.
    ///  * `max_type` - maximum value of the event type to be considered; [`SDL_EventType::First`]
    ///                 is a safe choice.
    ///
    /// # Return Value
    /// Returns the number of events actually stored or -1 on failure; call [`SDL_GetError`] for
    /// more information.
    ///
    /// # Remarks
    /// `action` may be any of the following:
    ///  - [`SDL_EventAction::AddEvent`]: up to `numevents` events will be added to the back of the
    ///                                   event queue.
    ///  - [`SDL_EventAction::PeekEvent`]: `numevents` events at the front of the event queue,
    ///                                    within the specified minimum and maximum type, will be
    ///                                    returned to the caller and will not be removed from the
    ///                                    queue. If you pass [`null_mut`] for events, then
    ///                                    `numevents` is ignored and the total number of matching
    ///                                    events will be returned.
    ///  - [`SDL_EventAction::GetEvent`]: up to `numevents` events at the front of the event queue,
    ///                                   within the specified minimum and maximum type, will be
    ///                                   returned to the caller and will be removed from the
    ///                                   queue.
    ///
    /// You may have to call [`SDL_PumpEvents`] before calling this function. Otherwise, the events
    /// may not be ready to be filtered when you call [`SDL_PeepEvents`].
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_PeepEvents(
        events: *mut SDL_Event,
        numevents: c_int,
        action: SDL_EventAction,
        min_type: u32,
        max_type: u32,
    ) -> c_int;
}
