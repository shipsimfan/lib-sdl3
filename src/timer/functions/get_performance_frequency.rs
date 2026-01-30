#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the count per second of the high resolution counter.
    ///
    /// # Return Value
    /// Returns a platform-specific count per second.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetPerformanceFrequency() -> u64;
}
