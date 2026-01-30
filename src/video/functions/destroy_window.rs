use crate::SDL_Window;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Destroy a window.
    ///
    /// # Parameters
    ///  * `window` - the window to destroy.
    ///
    /// # Remarks
    /// Any child windows owned by the window will be recursively destroyed as well.
    ///
    /// Note that on some platforms, the visible window may not actually be removed from the screen
    /// until the SDL event loop is pumped again, even though the [`SDL_Window`] is no longer valid
    /// after this call.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_DestroyWindow(window: *mut SDL_Window);
}
