// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetPerformanceFrequency;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get the current value of the high resolution counter.
    ///
    /// # Return Value
    /// Returns the current counter value.
    ///
    /// # Remarks
    /// This function is typically used for profiling.
    ///
    /// The counter values are only meaningful relative to each other. Differences between values
    /// can be converted to times by using [`SDL_GetPerformanceFrequency`].
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_GetPerformanceCounter() -> u64;
}
