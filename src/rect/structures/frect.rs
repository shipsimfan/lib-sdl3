use std::ffi::c_float;

/// A rectangle stored using floating point values.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_FRect {
    #[allow(missing_docs)]
    pub x: c_float,

    #[allow(missing_docs)]
    pub y: c_float,

    #[allow(missing_docs)]
    pub w: c_float,

    #[allow(missing_docs)]
    pub h: c_float,
}

impl Default for SDL_FRect {
    fn default() -> Self {
        SDL_FRect {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }
}
