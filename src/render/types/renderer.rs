use std::ffi::c_void;

/// A structure representing rendering state
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SDL_Renderer(c_void);
