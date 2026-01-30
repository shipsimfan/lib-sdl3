use std::ffi::c_float;

/// The structure that defines a point (using floating point values).
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_FPoint {
    #[allow(missing_docs)]
    pub x: c_float,

    #[allow(missing_docs)]
    pub y: c_float,
}

impl Default for SDL_FPoint {
    fn default() -> Self {
        SDL_FPoint { x: 0.0, y: 0.0 }
    }
}
