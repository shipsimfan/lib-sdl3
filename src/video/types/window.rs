use std::ffi::c_void;

/// The struct used as an opaque handle to a window.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SDL_Window(c_void);
