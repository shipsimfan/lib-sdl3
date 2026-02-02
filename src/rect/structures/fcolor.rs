use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PixelFormat;

/// The bits of this structure can be directly reinterpreted as a float-packed color which uses the
/// [`SDL_PixelFormat::RGBA128Float`] format
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_FColor {
    #[allow(missing_docs)]
    pub r: c_float,

    #[allow(missing_docs)]
    pub g: c_float,

    #[allow(missing_docs)]
    pub b: c_float,

    #[allow(missing_docs)]
    pub a: c_float,
}

impl Default for SDL_FColor {
    fn default() -> Self {
        SDL_FColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}
