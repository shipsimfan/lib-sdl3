#[link(name = "SDL3")]
extern "C" {
    /// Clear any previous error message for this thread.
    ///
    /// # Return Value
    /// Returns `true`.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    ///
    /// # Code Examples
    /// ```rust
    /// let error = SDL_GetError();
    /// if error != std::ptr::null() {
    ///     SDL_Log("SDL error: %s", error);
    ///     SDL_ClearError();
    /// }
    /// ```
    pub fn SDL_ClearError() -> bool;
}
