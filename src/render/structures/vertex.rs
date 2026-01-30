use crate::{SDL_FColor, SDL_FPoint};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_Renderer;

/// Vertex structure.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_Vertex {
    /// Vertex position, in [`SDL_Renderer`] coordinates
    pub position: SDL_FPoint,

    /// Vertex color
    pub color: SDL_FColor,

    /// Normalized texture coordinates, if needed
    pub tex_coord: SDL_FPoint,
}

impl Default for SDL_Vertex {
    fn default() -> Self {
        SDL_Vertex {
            position: SDL_FPoint::default(),
            color: SDL_FColor::default(),
            tex_coord: SDL_FPoint::default(),
        }
    }
}
