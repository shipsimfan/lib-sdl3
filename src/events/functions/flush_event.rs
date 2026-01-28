// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_FlushEvents, SDL_PeepEvents, SDL_PumpEvents};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Clear events of a specific type from the event queue.
    ///
    /// # Parameters
    ///  * `r#type` - the type of event to be cleared; see [`SDL_EventType`] for details.
    ///
    /// # Remarks
    /// This will unconditionally remove any events from the queue that match type. If you need to
    /// remove a range of event types, use [`SDL_FlushEvents`] instead.
    ///
    /// It's also normal to just ignore events you don't care about in your event loop without
    /// calling this function.
    ///
    /// This function only affects currently queued events. If you want to make sure that all
    /// pending OS events are flushed, you can call [`SDL_PumpEvents`] on the main thread
    /// immediately before the flush call.
    ///
    /// If you have user events with custom data that needs to be freed, you should use
    /// [`SDL_PeepEvents`] to remove and clean up those events before calling this function.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_FlushEvent(r#type: u32);
}
