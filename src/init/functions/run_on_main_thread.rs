use crate::SDL_MainThreadCallback;
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Call a function on the main thread during event processing.
    ///
    /// # Parameters
    ///  * `callback` - the callback to call on the main thread.
    ///  * `userdata` - a pointer that is passed to `callback`.
    ///  * `wait_complete` - `true` to wait for the callback to complete, `false` to return
    ///                      immediately.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// If this is called on the main thread, the callback is executed immediately. If this is
    /// called on another thread, this callback is queued for execution on the main thread during
    /// event processing.
    ///
    /// Be careful of deadlocks when using this functionality. You should not have the main thread
    /// wait for the current thread while this function is being called with `wait_complete`
    /// `true`.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_RunOnMainThread(
        callback: SDL_MainThreadCallback,
        userdata: *mut c_void,
        wait_complete: bool,
    ) -> bool;
}
