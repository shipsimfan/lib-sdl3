use std::ffi::c_float;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Compute the cosine of `x`.
    ///
    /// # Parameters
    ///  * `x` - floating point value, in radians.
    ///
    /// # Return Value
    /// Returns cosine of `x`.
    ///
    /// # Remarks
    /// Domain: `c_float::NEG_INFINITY <= x <= c_float::INFINITY`
    ///
    /// Range: `-1 <= y <= 1`
    ///
    /// This function operates on single-precision floating point values, use [`SDL_cos`] for
    /// double-precision floats.
    ///
    /// This function may use a different approximation across different versions, platforms and
    /// configurations. i.e, it can return a different value given the same input on different
    /// machines or operating systems, or if SDL is updated.
    pub fn SDL_cosf(x: c_float) -> c_float;
}
