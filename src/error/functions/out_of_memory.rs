#[link(name = "SDL3")]
extern "C" {
    /// Set an error indicating that memory allocation failed.
    ///
    /// # Return Value
    /// Returns `false`.
    ///
    /// # Remarks
    /// This function does not do any memory allocation.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_OutOfMemory() -> bool;
}
