#[link(name = "SDL3")]
unsafe extern "C" {
    /// Wait a specified number of milliseconds before returning.
    ///
    /// # Parameters
    ///  * `ms` - the number of milliseconds to delay.
    ///
    /// # Remarks
    /// This function waits a specified number of milliseconds before returning. It waits at least
    /// the specified time, but possibly longer due to OS scheduling.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_Delay(ms: u32);
}
