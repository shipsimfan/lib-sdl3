use crate::SDL_Renderer;
use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set the drawing scale for rendering on the current target.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `scale_x` - the horizontal scaling factor.
    ///  * `scale_y` - the vertical scaling factor.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// The drawing coordinates are scaled by the x/y scaling factors before they are used by the
    /// renderer. This allows resolution independent drawing with a single coordinate system.
    ///
    /// If this results in scaling or subpixel drawing by the rendering backend, it will be handled
    /// using the appropriate quality hints. For best results use integer scaling factors.
    ///
    /// Each render target has its own scale. This function sets the scale for the current render
    /// target.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetRenderScale(
        renderer: *mut SDL_Renderer,
        scale_x: c_float,
        scale_y: c_float,
    ) -> bool;
}
