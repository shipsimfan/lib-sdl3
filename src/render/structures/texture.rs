use crate::SDL_PixelFormat;
use std::ffi::c_int;

/// An efficient driver-specific representation of pixel data
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_Texture {
    /// The format of the texture, read-only
    pub format: SDL_PixelFormat,

    /// The width of the texture, read-only.
    pub w: c_int,

    /// The height of the texture, read-only.
    pub h: c_int,

    /// Application reference count, used when freeing texture
    pub refcount: c_int,
}

impl Default for SDL_Texture {
    fn default() -> Self {
        SDL_Texture {
            format: SDL_PixelFormat::Unknown,
            w: 0,
            h: 0,
            refcount: 0,
        }
    }
}
