use crate::{SDL_Renderer, SDL_RendererLogicalPresentation};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Set a device-independent resolution and presentation mode for rendering.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `w` - the width of the logical resolution.
    ///  * `h` - the height of the logical resolution.
    ///  * `mode` - the presentation mode used.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// This function sets the width and height of the logical rendering output. The renderer will
    /// act as if the current render target is always the requested dimensions, scaling to the
    /// actual resolution as necessary.
    ///
    /// This can be useful for games that expect a fixed size, but would like to scale the output
    /// to whatever is available, regardless of how a user resizes a window, or if the display is
    /// high DPI.
    ///
    /// Logical presentation can be used with both render target textures and the renderer's
    /// window; the state is unique to each render target, and this function sets the state for the
    /// current render target. It might be useful to draw to a texture that matches the window
    /// dimensions with logical presentation enabled, and then draw that texture across the entire
    /// window with logical presentation disabled. Be careful not to render both with logical
    /// presentation enabled, however, as this could produce double-letterboxing, etc.
    ///
    /// You can disable logical coordinates by setting the mode to
    /// [`SDL_RendererLogicalPresentation::Disabled`], and in that case you get the full pixel
    /// resolution of the render target; it is safe to toggle logical presentation during the
    /// rendering of a frame: perhaps most of the rendering is done to specific dimensions but to
    /// make fonts look sharp, the app turns off logical presentation while drawing text, for
    /// example.
    ///
    /// You can convert coordinates in an event into rendering coordinates using
    /// [`SDL_ConvertEventToRenderCoordinates`].
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_SetRenderLogicalPresentation(
        renderer: *mut SDL_Renderer,
        w: c_int,
        h: c_int,
        mode: SDL_RendererLogicalPresentation,
    ) -> bool;
}
