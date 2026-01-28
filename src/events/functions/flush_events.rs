// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_FlushEvent, SDL_PumpEvents};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Clear events of a range of types from the event queue.
    ///
    /// # Parameters
    ///  * `min_type` - the low end of event type to be cleared, inclusive; see [`SDL_EventType`]
    ///                 for details.
    ///  * `max_type` - the high end of event type to be cleared, inclusive; see [`SDL_EventType`]
    ///                 for details.
    ///
    /// # Remarks
    /// This will unconditionally remove any events from the queue that are in the range of
    /// `min_type` to `max_type`, inclusive. If you need to remove a single event type, use
    /// [`SDL_FlushEvent`] instead.
    ///
    /// It's also normal to just ignore events you don't care about in your event loop without
    /// calling this function.
    ///
    /// This function only affects currently queued events. If you want to make sure that all
    /// pending OS events are flushed, you can call [`SDL_PumpEvents`] on the main thread
    /// immediately before the flush call.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_FlushEvents(min_type: u32, max_type: u32);
}
