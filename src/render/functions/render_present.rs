use crate::SDL_Renderer;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_RenderClear};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Update the screen with any rendering performed since the previous call.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// SDL's rendering functions operate on a backbuffer; that is, calling a rendering function
    /// such as [`SDL_RenderLine`] does not directly put a line on the screen, but rather updates
    /// the backbuffer. As such, you compose your entire scene and present the composed backbuffer
    /// to the screen as a complete picture.
    ///
    /// Therefore, when using SDL's rendering API, one does all drawing intended for the frame, and
    /// then calls this function once per frame to present the final drawing to the user.
    ///
    /// The backbuffer should be considered invalidated after each present; do not assume that
    /// previous contents will exist between frames. You are strongly encouraged to call
    /// [`SDL_RenderClear`] to initialize the backbuffer before starting each new frame's drawing,
    /// even if you plan to overwrite every pixel.
    ///
    /// Please note, that in case of rendering to a texture - there is no need to call
    /// [`SDL_RenderPresent`] after drawing needed objects to a texture, and should not be done;
    /// you are only required to change back the rendering target to default via
    /// `SDL_SetRenderTarget(renderer, std::ptr::null_mut())` afterwards, as textures by themselves
    /// do not have a concept of backbuffers. Calling [`SDL_RenderPresent`] while rendering to a
    /// texture will fail.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer) -> bool;
}
