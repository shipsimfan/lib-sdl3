// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PeepEvents;

/// The type of action to request from [`SDL_PeepEvents`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_EventAction {
    /// Add events to the back of the queue.
    AddEvent,

    /// Check but don't remove events from the queue front.
    PeekEvent,

    /// Retrieve/remove events from the front of the queue.
    GetEvent,
}
