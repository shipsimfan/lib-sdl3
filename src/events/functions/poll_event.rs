use crate::SDL_Event;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PumpEvents;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Poll for currently pending events.
    ///
    /// # Parameters
    ///  * `event` - the [`SDL_Event`] structure to be filled with the next event from the queue,
    ///              or [`null_mut`].
    ///
    /// # Return Value
    /// Returns `true` if this got an event or `false` if there are none available.
    ///
    /// # Remarks
    /// If `event` is not [`null_mut`], the next event is removed from the queue and stored in the
    /// [`SDL_Event`] structure pointed to by event. The 1 returned refers to this event,
    /// immediately stored in the SDL Event structure -- not an event to follow.
    ///
    /// If `event` is [`null_mut`], it simply returns 1 if there is an event in the queue, but will
    /// not remove it from the queue.
    ///
    /// As this function may implicitly call [`SDL_PumpEvents`], you can only call this function in
    /// the thread that set the video mode.
    ///
    /// [`SDL_PollEvent`] is the favored way of receiving system events since it can be done from
    /// the main loop and does not suspend the main loop while waiting on an event to be posted.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> bool;
}
