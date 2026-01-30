use crate::SDL_Renderer;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Destroy the rendering context for a window and free all associated textures.
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///
    /// # Remarks
    /// This should be called before destroying the associated window.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);
}
