use crate::SDL_Color;
use std::{ffi::c_int, ptr::null_mut};

/// A set of indexed colors representing a palette.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_Palette {
    /// number of elements in `colors`.
    pub n_colors: c_int,

    /// an array of colors, `n_colors` long.
    pub colors: *mut SDL_Color,

    /// internal use only, do not touch.
    pub version: i32,

    /// internal use only, do not touch.
    pub refcount: c_int,
}

impl Default for SDL_Palette {
    fn default() -> Self {
        SDL_Palette {
            n_colors: 0,
            colors: null_mut(),
            version: 0,
            refcount: 0,
        }
    }
}
