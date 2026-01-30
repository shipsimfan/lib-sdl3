use std::ffi::c_int;

/// A rectangle, with the origin at the upper left (using integers).
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_Rect {
    #[allow(missing_docs)]
    pub x: c_int,

    #[allow(missing_docs)]
    pub y: c_int,

    #[allow(missing_docs)]
    pub w: c_int,

    #[allow(missing_docs)]
    pub h: c_int,
}

impl Default for SDL_Rect {
    fn default() -> Self {
        SDL_Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}
