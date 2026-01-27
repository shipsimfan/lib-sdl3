use crate::{SDL_EventType, SDL_MouseButtonFlags, SDL_MouseID, SDL_WindowID};
use std::ffi::c_float;

/// Mouse motion event structure (event.motion.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_MouseMotionEvent {
    /// [`SDL_EventType::MouseMotion`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with mouse focus, if any
    pub window_id: SDL_WindowID,

    /// The mouse instance id in relative mode, SDL_TOUCH_MOUSEID for touch events, or 0
    pub which: SDL_MouseID,

    /// The current button state
    pub state: SDL_MouseButtonFlags,

    /// The current button state
    pub x: c_float,

    /// Y coordinate, relative to window
    pub y: c_float,

    /// The relative motion in the X direction
    pub xrel: c_float,

    /// The relative motion in the Y direction
    pub yrel: c_float,
}

impl Default for SDL_MouseMotionEvent {
    fn default() -> Self {
        SDL_MouseMotionEvent {
            r#type: SDL_EventType::MouseMotion,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            state: 0,
            x: 0.0,
            y: 0.0,
            xrel: 0.0,
            yrel: 0.0,
        }
    }
}
