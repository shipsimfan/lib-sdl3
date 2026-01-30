use std::{ffi::CStr, ptr::null_mut, time::Duration};

fn main() {
    if !unsafe { sdl3::SDL_Init(0) } {
        eprintln!(
            "error initializing SDL: {}",
            unsafe { CStr::from_ptr(sdl3::SDL_GetError()) }.to_string_lossy()
        );
        return;
    }

    let window = unsafe { sdl3::SDL_CreateWindow(c"Blank Window Example".as_ptr(), 1280, 720, 0) };
    if window == null_mut() {
        eprintln!(
            "error creating window: {}",
            unsafe { CStr::from_ptr(sdl3::SDL_GetError()) }.to_string_lossy()
        );
        unsafe { sdl3::SDL_Quit() };
        return;
    }

    std::thread::sleep(Duration::from_secs(5));

    unsafe { sdl3::SDL_DestroyWindow(window) };
    unsafe { sdl3::SDL_Quit() };
}
