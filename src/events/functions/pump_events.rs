// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_PollEvent, SDL_WaitEvent};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Pump the event loop, gathering events from the input devices.
    ///
    /// # Remarks
    /// This function updates the event queue and internal input device state.
    ///
    /// [`SDL_PumpEvents`] gathers all the pending input information from devices and places it in
    /// the event queue. Without calls to [`SDL_PumpEvents`] no events would ever be placed on the
    /// queue. Often the need for calls to [`SDL_PumpEvents`] is hidden from the user since
    /// [`SDL_PollEvent`] and [`SDL_WaitEvent`] implicitly call [`SDL_PumpEvents`]. However, if you
    /// are not polling or waiting for events (e.g. you are filtering them), then you must call
    /// [`SDL_PumpEvents`] to force an event queue update.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_PumpEvents();
}
