use crate::{SDL_Renderer, SDL_Texture, SDL_Vertex};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Render a list of triangles, optionally using a texture and indices into the vertex array
    /// Color and alpha modulation is done per vertex ([`SDL_SetTextureColorMod`] and
    /// [`SDL_SetTextureAlphaMod`] are ignored).
    ///
    /// # Parameters
    ///  * `renderer` - the rendering context.
    ///  * `texture` - (optional) The SDL texture to use.
    ///  * `vertices` - vertices.
    ///  * `num_vertices` - number of vertices.
    ///  * `indices` - (optional) An array of integer indices into the 'vertices' array, if
    ///                [`null`] all vertices will be rendered in sequential order.
    ///  * `num_indices` - number of indices.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Thread Safety
    /// This function should only be called on the main thread.
    pub fn SDL_RenderGeometry(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        vertices: *const SDL_Vertex,
        num_vertices: c_int,
        indices: *const c_int,
        num_indices: c_int,
    ) -> bool;
}
