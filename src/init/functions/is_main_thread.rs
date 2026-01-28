#[link(name = "SDL3")]
unsafe extern "C" {
    /// Return whether this is the main thread.
    ///
    /// # Return Value
    /// Returns `true` if this thread is the main thread, or `false` otherwise.
    ///
    /// # Remarks
    /// On Apple platforms, the main thread is the thread that runs your program's `main` entry
    /// point. On other platforms, the main thread is the one that calls
    /// `SDL_Init(SDL_INIT_VIDEO)`, which should usually be the one that runs your program's `main`
    /// entry point. If you are using the main callbacks, [`SDL_AppInit`], [`SDL_AppIterate`], and
    /// [`SDL_AppQuit`] are all called on the main thread.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_IsMainThread() -> bool;
}
